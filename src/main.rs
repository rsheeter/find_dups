use std::{collections::HashMap, f64::consts::PI, fs, path::Path};

use clap::{command, Parser};
use kurbo::{Affine, BezPath, ParamCurve, ParamCurveNearest, PathEl, Point, Rect, Shape};
use skrifa::{instance::Size, raw::TableProvider, FontRef, MetadataProvider};
use write_fonts::pens::BezPathPen;

const DEFAULT_TEST_STRING: &str =
    r#"1234567890-=!@#$%^&*()_+qWeRtYuIoP[]|AsDfGhJkL:"zXcVbNm,.<>{}[]üøéåîÿçñè"#;

/// Seems shockingly high but reflects actual observed results
///
/// For example, nearest (104.06, -416.96) is (103.32081258597977, -416.74961307615433), 0.77 apart
const UPEM_EPSILON: f64 = 15.0;
const NEAREST_EPSILON: f64 = 0.0000001;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Compare these characters to detect duplication
    #[arg(short, long)]
    test_string: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    files: Vec<String>,
}

trait AboutTheSame<T = Self> {
    fn approximately_equal(&self, other: &T) -> bool;
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
    fn approximately_equal(&self, other: &Self) -> bool {
        for segment in self.segments() {
            for t in 0..=10 {
                let t = t as f64 / 10.0;
                let pt_self = segment.eval(t);
                let pt_other = nearest(pt_self, other);
                let separation = (pt_self - pt_other).length();
                if separation > UPEM_EPSILON {
                    eprintln!("Nearest {pt_self:?} is {pt_other:?}, {separation:.2} apart",);
                    return false;
                }
            }
        }
        // Failed to find any point whose nearest on other was too far away
        true
    }
}

fn oncurve_points(path: &BezPath) -> Vec<Point> {
    let mut last_start = None;
    let mut result = Vec::with_capacity(path.elements().len());
    for el in path.elements() {
        match el {
            PathEl::MoveTo(end) => {
                last_start = Some(*end);
                result.push(*end);
            }
            PathEl::LineTo(end) | PathEl::QuadTo(_, end) | PathEl::CurveTo(.., end) => {
                result.push(*end)
            }
            PathEl::ClosePath => {
                result.push(last_start.unwrap_or_else(|| panic!("Malformed path")))
            }
        }
    }
    result
}

fn svg_circle(x: f64, y: f64, r: f64) -> String {
    format!("<circle fill=\"darkblue\" opacity=\"0.25\" cx=\"{x}\" cy=\"{y}\" r=\"{r}\" />\n")
}

fn main() {
    let args = Args::parse();

    let paths: Vec<_> = args
        .files
        .iter()
        .filter_map(|f| {
            let file = Path::new(f);
            if !file.is_file() {
                eprintln!("{file:?} is not a file");
                return None;
            }
            Some(file)
        })
        .collect();
    let raw_fonts: Vec<_> = paths.iter().map(|p| fs::read(p).unwrap()).collect();
    let fonts: Vec<_> = raw_fonts
        .iter()
        .zip(&paths)
        .map(|(bytes, path)| {
            FontRef::new(bytes).unwrap_or_else(|e| panic!("Unable to load {path:?}: {e}"))
        })
        .collect();

    if fonts.is_empty() {
        eprintln!("Not much to do with no fonts specified");
        return;
    }

    // we will scale to the largest upem
    let max_upem = fonts
        .iter()
        .map(|f| f.head().unwrap().units_per_em())
        .max()
        .unwrap();
    let test_chars = args
        .test_string
        .as_deref()
        .unwrap_or(DEFAULT_TEST_STRING)
        .chars()
        .collect::<Vec<_>>();
    let mut glyphs: HashMap<char, Vec<BezPath>> = Default::default();

    // Really we should shape the test string but we don't have a safe shaper.
    // This should suffice for copied Latin which is our primarily use case.
    for font in fonts.iter() {
        let upem = font.head().unwrap().units_per_em();
        let uniform_scale = if upem != max_upem {
            max_upem as f64 / upem as f64
        } else {
            1.0
        };
        let transform = Affine::scale_non_uniform(uniform_scale, -uniform_scale);
        let cmap = font.cmap().unwrap();
        let outlines = font.outline_glyphs();

        for c in test_chars.iter() {
            let mut path = BezPath::default();

            if let Some(gid) = cmap.map_codepoint(*c) {
                let glyph = outlines.get(gid).unwrap();
                let mut pen = BezPathPen::new();
                glyph.draw(Size::unscaled(), &mut pen).unwrap();
                path = pen.into_inner();
                path.apply_affine(transform);
            }
            glyphs.entry(*c).or_default().push(path);
        }
    }

    // We have every char for every font scaled to a common upem; are they the same?
    let mut results: HashMap<bool, Vec<char>> = Default::default();
    for c in test_chars.iter() {
        let paths = glyphs.get(c).unwrap();
        let first_path = &paths.first().unwrap();
        let consistent = paths.iter().all(|p| first_path.approximately_equal(p));
        results.entry(consistent).or_default().push(*c);
    }
    for (consistent, chars) in results.iter() {
        let prefix = if *consistent {
            "Consistent"
        } else {
            "Inconsistent"
        };
        eprintln!(
            "{} {}/{}: {}",
            prefix,
            chars.len(),
            test_chars.len(),
            chars.iter().cloned().collect::<String>()
        );
    }

    if let Some(inconsistent) = results.get(&false) {
        for c in inconsistent {
            let mut viewbox = Rect::new(0.0, 0.0, 0.0, 0.0);
            let mut svg = String::new();
            let marker_radius = max_upem as f64 * 0.01;
            for path in glyphs.get(c).unwrap() {
                // actual path
                svg.push_str(
                    format!("<path opacity=\"0.25\" d=\"{}\" />\n", path.to_svg()).as_str(),
                );
            }
            for path in glyphs.get(c).unwrap() {
                // start marker
                if let Some(PathEl::MoveTo(p)) = path.elements().first() {
                    svg.push_str(svg_circle(p.x, p.y, marker_radius).as_str());
                }
                // direction markers
                for pair in oncurve_points(path).windows(2) {
                    // TODO fix for curves by drawing at t=0.5 in the direction of the tangent

                    let mid = pair[0].midpoint(pair[1]);
                    let backtrack = (pair[0] - mid).normalize() * marker_radius;
                    let p0 = mid + (Affine::rotate(PI / 4.0) * backtrack.to_point()).to_vec2();
                    let p1 = mid + (Affine::rotate(-PI / 4.0) * backtrack.to_point()).to_vec2();

                    let mut marker_path = BezPath::new();
                    marker_path.move_to(mid);
                    marker_path.line_to(p0);
                    marker_path.line_to(p1);
                    marker_path.close_path();

                    // svg.push_str(
                    //     format!("<path opacity=\"0.25\" d=\"{}\" />\n", marker_path.to_svg()).as_str(),
                    // );
                }

                viewbox = viewbox.union(path.bounding_box());
            }
            let margin = 0.1 * viewbox.width().max(viewbox.height());
            svg = format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">\n{}\n</svg>",
                viewbox.min_x() - margin,
                viewbox.min_y() - margin,
                viewbox.width() + 2.0 * margin,
                viewbox.height() + 2.0 * margin,
                svg
            );
            fs::write(format!("inconsistent-{c}.svg"), svg).unwrap();
        }
    }
}
