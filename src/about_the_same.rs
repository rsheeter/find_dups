//! Helpers for comparing [`BezPath`]

use std::sync::atomic::AtomicUsize;

use kurbo::{BezPath, Line, ParamCurve, ParamCurveNearest, PathSeg, Point, Rect};
use thiserror::Error;

const NEAREST_EPSILON: f64 = 0.0000001;

#[derive(Debug, Clone, Copy)]
pub struct RulesOfSimilarity {
    pub equivalence: f64,
    pub budget: f64,
    pub error: f64,
}

impl RulesOfSimilarity {
    pub fn for_upem(self, upem: u16) -> Self {
        if upem == 1000 {
            return self;
        };
        let scale = upem as f64 / 1000.0;
        Self {
            equivalence: self.equivalence * scale,
            budget: self.budget * scale,
            error: self.error * scale,
        }
    }
}

#[derive(Error, Debug)]
pub enum ApproximatelyEqualError {
    #[error("{separation:.2} exceeds error limit. {rules:?}.")]
    BrokeTheHardDeck {
        separation: f64,
        rules: RulesOfSimilarity,
    },
    #[error("Exhaused budget. {0:?}.")]
    ExhaustedBudget(RulesOfSimilarity),
    #[error("One of Self and other is empty")]
    EmptinessMismatch,
}

pub trait AboutTheSame<T = Self> {
    fn approximately_equal(
        &self,
        other: &T,
        rules: RulesOfSimilarity,
    ) -> Result<(), ApproximatelyEqualError>;
}

fn control_box(s: PathSeg) -> Rect {
    match s {
        PathSeg::Line(line) => Rect::from_points(line.p0, line.p1),
        PathSeg::Quad(quad) => Rect::from_points(quad.p0, quad.p1).union_pt(quad.p2),
        PathSeg::Cubic(cubic) => Rect::from_points(cubic.p0, cubic.p1)
            .union_pt(cubic.p2)
            .union_pt(cubic.p3),
    }
}

/// How many times nearest was called. Helpful when trying to make # smaller.
pub static NUM_NEAREST: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone)]
struct PotentialNearness {
    min_dst_sq: f64,
    max_dst_sq: f64,
    precomp: PrecomputedSegment,
}

fn corners(r: Rect) -> [Point; 4] {
    [
        Point::new(r.x0, r.y0),
        Point::new(r.x0, r.y1),
        Point::new(r.x1, r.y0),
        Point::new(r.x1, r.y1),
    ]
}

fn lines(corners: [Point; 4]) -> [Line; 4] {
    [
        Line::new(corners[0], corners[1]),
        Line::new(corners[1], corners[2]),
        Line::new(corners[2], corners[3]),
        Line::new(corners[3], corners[0]),
    ]
}

impl PotentialNearness {
    fn new(p: Point, segment: PrecomputedSegment) -> Self {
        let mut min_dst_sq = 0.0;
        let max_dst_sq = segment
            .corners
            .iter()
            .map(|c| (*c - p).length())
            .reduce(f64::max)
            .unwrap()
            .powf(2.0);
        if !segment.control_box.contains(p) {
            min_dst_sq = segment
                .lines
                .iter()
                .map(|l| l.nearest(p, NEAREST_EPSILON).distance_sq)
                .reduce(f64::min)
                .unwrap();
        }
        Self {
            min_dst_sq,
            max_dst_sq,
            precomp: segment,
        }
    }

    fn closer(&self, other: PotentialNearness) -> bool {
        self.max_dst_sq < other.min_dst_sq
    }

    fn intersects(&self, other: PotentialNearness) -> bool {
        self.max_dst_sq >= other.min_dst_sq && self.min_dst_sq <= other.max_dst_sq
    }
}

// Computing nearest for every segment and reducing was very slow
fn nearest(scratch: &mut Vec<PotentialNearness>, p: Point, other: &GlyphPath) -> Point {
    scratch.clear();
    for segment in other.segments.iter() {
        let nearness = PotentialNearness::new(p, *segment);
        if scratch.iter().any(|n| n.closer(nearness)) {
            continue; // already assured a better result
        }
        scratch.retain(|n| n.intersects(nearness));
        scratch.push(nearness);
    }
    scratch
        .iter()
        .map(|n| {
            let nearest = n.precomp.segment.nearest(p, NEAREST_EPSILON);
            NUM_NEAREST.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            (nearest.distance_sq, n.precomp.segment.eval(nearest.t))
        })
        .reduce(|acc, e| if acc.0 <= e.0 { acc } else { e })
        .expect("Don't use this with empty paths")
        .1
}

#[derive(Debug, Clone, Copy)]
pub struct PrecomputedSegment {
    segment: PathSeg,
    control_box: Rect,
    corners: [Point; 4],
    lines: [Line; 4],
}

impl PrecomputedSegment {
    fn new(segment: PathSeg) -> Self {
        let control_box = control_box(segment);
        let corners = corners(control_box);
        let lines = lines(corners);
        Self {
            segment,
            control_box,
            corners,
            lines,
        }
    }
}

/// A BezPath with segments, segment bboxes, etc precomputed
#[derive(Debug, Clone)]
pub struct GlyphPath {
    pub path: BezPath,
    pub segments: Vec<PrecomputedSegment>,
}

impl GlyphPath {
    pub fn new(path: BezPath) -> Self {
        let segments = path
            .segments()
            .map(|s| PrecomputedSegment::new(s))
            .collect();
        Self { path, segments }
    }
}

impl AboutTheSame for GlyphPath {
    /// Meant to work with non-adversarial, similar, curves like letterforms
    ///
    /// Think the same I drawn with two different sets of drawing commands    
    fn approximately_equal(
        &self,
        other: &Self,
        rules: RulesOfSimilarity,
    ) -> Result<(), ApproximatelyEqualError> {
        let mut budget = rules.budget;

        if self.path.is_empty() != other.path.is_empty() {
            return Err(ApproximatelyEqualError::EmptinessMismatch);
        }

        let mut scratch = Vec::with_capacity(4);

        for precomp in self.segments.iter() {
            for t in 0..=10 {
                let t = t as f64 / 10.0;
                let pt_self = precomp.segment.eval(t);
                let pt_other = nearest(&mut scratch, pt_self, other);
                let separation = (pt_self - pt_other).length();

                if separation <= rules.equivalence {
                    continue;
                }
                if separation > rules.error {
                    return Err(ApproximatelyEqualError::BrokeTheHardDeck { separation, rules });
                }
                budget -= separation.powf(2.0);
                log::debug!("Nearest {pt_self:?} is {pt_other:?}, {separation:.2} apart. {}/{} budget remains.", budget, rules.budget);
                if budget < 0.0 {
                    log::debug!("Fail due to exhausted budget");
                    return Err(ApproximatelyEqualError::ExhaustedBudget(rules));
                }
            }
        }
        Ok(())
    }
}
