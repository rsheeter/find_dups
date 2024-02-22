use std::{collections::HashMap, f64::consts::PI, fs, path::Path};

use clap::{command, Parser};
use kurbo::{Affine, BezPath, PathEl, Point, Rect, Shape};
use skrifa::{instance::Size, raw::TableProvider, FontRef, MetadataProvider};
use write_fonts::pens::BezPathPen;

const DEFAULT_TEST_STRING: &str =
    r#"1234567890-=!@#$%^&*()_+qWeRtYuIoP[]|AsDfGhJkL:"zXcVbNm,.<>{}[]üøéåîÿçñè"#;

const UPEM_EPSILON: f64 = 0.001;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Compare these characters to detect duplication
    #[arg(short, long)]
    test_string: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    files: Vec<String>,
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

fn normalize(debug: &str, paths: &mut Vec<BezPath>) {
    let Some(first) = paths.first() else {
        return;
    };

    // TODO: process subpath by subpath

    if first.elements().len() < 2 {
        return;
    }
    // TODO: We want to start at desired[0] and move in the direction of desired[1]
    // Right now we could end up reversed and not notice
    let desired_oncurve = oncurve_points(first);

    for i in 1..paths.len() {
        let path = &paths[i];
        assert!(matches!(path.elements().first(), Some(PathEl::MoveTo(..))));

        let points = oncurve_points(path);
        let Some(start_idx) = points
            .iter()
            .position(|p| (*p - desired_oncurve[0]).length() < UPEM_EPSILON)
        else {
            eprintln!("{debug} start not present in points?!");
            continue;
        };
        if start_idx == 0 {
            continue;
        }

        let mut fixed = Vec::new();
        // add elements back in the new order
        fixed.push(PathEl::MoveTo(desired_oncurve[0]));
        for i in start_idx..(start_idx + path.elements().len()) {
            let el_idx = i % path.elements().len();
            let el = path.elements()[el_idx];
            match el {
                PathEl::ClosePath => fixed.push(PathEl::LineTo(points[el_idx])),
                PathEl::MoveTo(..) => (),
                _ => fixed.push(el),
            }
        }
        fixed.push(PathEl::ClosePath);

        paths[i] = BezPath::from_vec(fixed);
    }
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
    for (path, font) in paths.iter().zip(fonts) {
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

    // In a fascinating turn of events it seems we sometimes have the same path with a different start point
    for (c, paths) in glyphs.iter_mut() {
        normalize(format!("{c}").as_str(), paths);
    }

    // We have every char for every font scaled to a common upem; are they the same?
    let mut results: HashMap<bool, Vec<char>> = Default::default();
    for c in test_chars.iter() {
        let paths = glyphs.get(c).unwrap();
        let first_path = &paths.first().unwrap();
        let consistent = paths.iter().all(|p| *first_path == p);
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
