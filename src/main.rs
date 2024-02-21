use std::{collections::HashMap, fs, path::Path};

use clap::{command, Parser};
use kurbo::{Affine, BezPath, Rect, Shape};
use skrifa::{instance::Size, outline::OutlinePen, raw::TableProvider, FontRef, MetadataProvider};

const DEFAULT_TEST_STRING: &str =
    r#"1234567890-=!@#$%^&*()_+qWeRtYuIoP[]|AsDfGhJkL:"zXcVbNm,.<>{}[]üøéåîÿçñè"#;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Compare these characters to detect duplication
    #[arg(short, long)]
    test_string: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    files: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct GlyphPath {
    path: BezPath,
}

impl OutlinePen for GlyphPath {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to((x as f64, y as f64));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x as f64, y as f64));
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.path
            .quad_to((cx0 as f64, cy0 as f64), (x as f64, y as f64));
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.path.curve_to(
            (cx0 as f64, cy0 as f64),
            (cx1 as f64, cy1 as f64),
            (x as f64, y as f64),
        );
    }

    fn close(&mut self) {
        self.path.close_path()
    }
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
    let mut glyphs = Vec::new();

    // Really we should shape the test string but we don't have a safe shaper.
    // This should suffice for copied Latin which is our primarily use case.
    for (path, font) in paths.iter().zip(fonts) {
        let upem = font.head().unwrap().units_per_em();
        let scale = (upem != max_upem).then(|| Affine::scale(max_upem as f64 / upem as f64));
        let cmap = font.cmap().unwrap();
        let outlines = font.outline_glyphs();

        if let Some(scale) = scale {
            eprintln!("Scaling {path:?} by {scale:?}");
        }

        glyphs.push(Vec::new());
        for c in test_chars.iter() {
            let mut glyph_path = GlyphPath::default();

            if let Some(gid) = cmap.map_codepoint(*c) {
                let glyph = outlines.get(gid).unwrap();
                glyph.draw(Size::unscaled(), &mut glyph_path).unwrap();
                if let Some(scale) = scale {
                    glyph_path.path.apply_affine(scale);
                }
            }
            glyphs.last_mut().unwrap().push(glyph_path);
        }
    }

    // We have every char for every font scaled to a common upem; are they the same?
    let mut results: HashMap<bool, Vec<char>> = Default::default();
    for (i, c) in test_chars.iter().enumerate() {
        let first_path = &glyphs.first().unwrap()[i];
        let consistent = glyphs
            .iter()
            .map(|paths| &paths[i])
            .all(|p| first_path == p);
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
            let i = test_chars.iter().position(|tc| tc == c).unwrap();
            for path in glyphs.iter().map(|paths| &paths[i]) {
                svg.push_str(
                    format!("<path opacity=\"0.25\" d=\"{}\" />\n", path.path.to_svg()).as_str(),
                );
                viewbox = viewbox.union(path.path.bounding_box());
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
