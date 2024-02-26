//! Helpers for comparing [`BezPath`]

use kurbo::{BezPath, ParamCurve, ParamCurveNearest, Point};
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

fn nearest(p: Point, other: &BezPath) -> Point {
    other
        .segments()
        .map(|s| {
            let nearest = s.nearest(p, NEAREST_EPSILON);
            (nearest.distance_sq, s.eval(nearest.t))
        })
        .reduce(|acc, e| if acc.0 <= e.0 { acc } else { e })
        .expect("Don't use this with empty paths")
        .1
}

impl AboutTheSame for BezPath {
    /// Meant to work with non-adversarial, similar, curves like letterforms
    ///
    /// Think the same I drawn with two different sets of drawing commands    
    fn approximately_equal(
        &self,
        other: &Self,
        rules: RulesOfSimilarity,
    ) -> Result<(), ApproximatelyEqualError> {
        let mut budget = rules.budget;

        if self.is_empty() != other.is_empty() {
            return Err(ApproximatelyEqualError::EmptinessMismatch);
        }

        for segment in self.segments() {
            for t in 0..=10 {
                let t = t as f64 / 10.0;
                let pt_self = segment.eval(t);
                let pt_other = nearest(pt_self, other);
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
