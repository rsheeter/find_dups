use std::{
    collections::{BTreeSet, HashMap},
    fs, io,
    path::{self, Path, PathBuf},
};

use clap::Parser;
use kurbo::{Affine, BezPath, PathEl, Shape};
use skrifa::{instance::Size, raw::TableProvider, FontRef, MetadataProvider};
use write_fonts::pens::BezPathPen;

use find_dups::{
    about_the_same::{AboutTheSame, ApproximatelyEqualError, RulesOfSimilarity},
    args::Args,
};

fn svg_circle(x: f64, y: f64, r: f64) -> String {
    format!("<circle fill=\"darkblue\" opacity=\"0.25\" cx=\"{x}\" cy=\"{y}\" r=\"{r}\" />\n")
}

fn init_logging() {
    use std::io::Write;
    env_logger::builder()
        .format(|buf, record| {
            let ts = buf.timestamp_micros();
            writeln!(
                buf,
                "[{ts} {} {} {}] {}",
                // we manually assign all threads a name
                std::thread::current().name().unwrap_or("unknown"),
                record.target(),
                buf.default_level_style(record.level())
                    .value(record.level()),
                record.args()
            )
        })
        .init();
}

fn load_fonts<'a>(
    paths: impl Iterator<Item = &'a Path>,
) -> Result<HashMap<PathBuf, Vec<u8>>, io::Error> {
    paths
        .map(|p| Ok((p.to_path_buf(), fs::read(p)?)))
        .collect::<Result<_, _>>()
}

struct LetterformGroup<'a> {
    letterforms: HashMap<&'a Path, Letterform>,
}

impl<'a> LetterformGroup<'a> {
    fn new(path: &'a Path, letterform: Letterform) -> Self {
        Self {
            letterforms: HashMap::from([(path, letterform)]),
        }
    }

    fn matches(&self, letterform: &Letterform, rules: RulesOfSimilarity) -> bool {
        self.letterforms
            .values()
            .any(|l| l.approximately_equal(letterform, rules).is_ok())
    }

    fn insert(&mut self, path: &'a Path, letterform: Letterform) -> Option<Letterform> {
        self.letterforms.insert(path, letterform)
    }
}

#[derive(Debug, Clone)]
struct Letterform(BezPath);

impl AboutTheSame for Letterform {
    fn approximately_equal(
        &self,
        other: &Self,
        rules: RulesOfSimilarity,
    ) -> Result<(), ApproximatelyEqualError> {
        self.0.approximately_equal(&other.0, rules)
    }
}

impl Letterform {
    fn create(font: &FontRef, c: char, uniform_scale: f64) -> Self {
        let transform = Affine::scale_non_uniform(uniform_scale, -uniform_scale);
        let cmap = font.cmap().unwrap();
        let outlines = font.outline_glyphs();

        let mut path = BezPath::default();
        if let Some(gid) = cmap.map_codepoint(c) {
            let glyph = outlines.get(gid).unwrap();
            let mut pen = BezPathPen::new();
            glyph.draw(Size::unscaled(), &mut pen).unwrap();
            path = pen.into_inner();
            path.apply_affine(transform);

            // plant the control box at 0,0 so translation doesn't cause mismatches
            let cbox = path.control_box();
            let (minx, miny) = (cbox.min_x(), cbox.min_y());
            if (minx, miny) != (0.0, 0.0) {
                path.apply_affine(Affine::translate((-minx, -miny)));
            }
        }
        Self(path)
    }
}

fn letterforms<'a>(groups: &'a [LetterformGroup]) -> impl Iterator<Item = &'a Letterform> {
    groups.iter().flat_map(|g| g.letterforms.values())
}

fn path_safe_c(c: char) -> String {
    if path::is_separator(c) {
        format!("0x{:04x}x", c as u32)
    } else {
        format!("{c}")
    }
}

fn dump_glyphs(working_dir: &Path, all_letterforms: &HashMap<char, Vec<LetterformGroup>>) {
    for (c, group) in all_letterforms.iter() {
        let viewbox = letterforms(group)
            .map(|l| l.0.bounding_box())
            .reduce(|acc, e| acc.union(e))
            .unwrap_or_default();
        let marker_radius = viewbox.width() * 0.02;
        let margin = 0.1 * viewbox.width().max(viewbox.height());

        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">\n",
            viewbox.min_x() - margin,
            viewbox.min_y() - margin,
            viewbox.width() + 2.0 * margin,
            viewbox.height() + 2.0 * margin,
        );
        for path in letterforms(group).map(|l| &l.0) {
            // actual path
            svg.push_str(format!("<path opacity=\"0.25\" d=\"{}\" />\n", path.to_svg()).as_str());
        }
        for path in letterforms(group).map(|l| &l.0) {
            // start marker
            if let Some(PathEl::MoveTo(p)) = path.elements().first() {
                svg.push_str(svg_circle(p.x, p.y, marker_radius).as_str());
            }
        }

        svg.push_str("</svg>\n");
        let suffix = if group.len() > 1 { "-inconsistent" } else { "" };
        let c = path_safe_c(*c);
        let dest = working_dir.join(format!("glyph_{c}{suffix}.svg"));
        fs::write(&dest, svg).unwrap_or_else(|e| panic!("Unable to write {dest:?}: {e}"));
    }
}

fn dump_groups(working_dir: &Path, all_letterforms: &HashMap<char, Vec<LetterformGroup>>) {
    for (c, groups) in all_letterforms.iter() {
        for (i, group) in groups.iter().enumerate() {
            let mut paths = group
                .letterforms
                .keys()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>();
            paths.sort();
            let mut content = format!("{} files with matching {c}\n", paths.len());
            for path in paths {
                content.push_str(path);
                content.push('\n');
            }
            let c = path_safe_c(*c);
            let dest = working_dir.join(format!("group_{c}.{i}.txt"));
            fs::write(&dest, content).unwrap_or_else(|e| panic!("Unable to write {dest:?}: {e}"));
        }
    }
}

fn log_groups(test_chars: &[char], letterforms: &HashMap<char, Vec<LetterformGroup>>) {
    if !log::log_enabled!(log::Level::Debug) {
        return;
    }
    for c in test_chars.iter() {
        let groups = letterforms
            .get(c)
            .expect("All test chars should be defined");
        log::debug!("{} groups for '{c}'", groups.len());
        for (i, group) in groups.iter().enumerate() {
            log::debug!(
                "  {i}: {:?}",
                group
                    .letterforms
                    .keys()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
            );
        }
    }
}

fn dump_stuff(args: &Args, letterforms: &HashMap<char, Vec<LetterformGroup>>) {
    let working_dir = Path::new(&args.working_dir);
    if working_dir.is_dir() {
        for del_pat in ["*.svg", "*.txt"] {
            for file in
                glob::glob(working_dir.join(del_pat).to_str().expect("Oh no")).expect("To glob")
            {
                let file = file.expect("Access to working dir");
                fs::remove_file(file).expect("To be able to delete working dir files");
            }
        }
    } else {
        fs::create_dir(working_dir).unwrap();
    }
    if args.dump_glyphs {
        dump_glyphs(working_dir, letterforms);
    }
    if args.dump_groups {
        dump_groups(working_dir, letterforms);
    }
}

fn create_grouped_letterforms<'a>(
    rules: RulesOfSimilarity,
    test_chars: &[char],
    raw_fonts: &'a HashMap<PathBuf, Vec<u8>>,
) -> Result<HashMap<char, Vec<LetterformGroup<'a>>>, ()> {
    let fonts: HashMap<_, _> = raw_fonts
        .iter()
        .map(|(path, bytes)| {
            (
                path,
                FontRef::new(bytes).unwrap_or_else(|e| panic!("Unable to load {path:?}: {e}")),
            )
        })
        .collect();

    if fonts.is_empty() {
        log::error!("Not much to do with no fonts specified");
        return Err(());
    }

    // we will scale to the largest upem
    let max_upem = fonts
        .values()
        .map(|f| f.head().unwrap().units_per_em())
        .max()
        .unwrap();

    // budget is based on 1000 upem; scale if necessary
    let rules = rules.for_upem(max_upem);
    log::info!("The rules are {rules:?}");
    let mut glyphs: HashMap<char, Vec<BezPath>> = Default::default();

    // Really we should shape the test string but we don't have a safe shaper.
    // This should suffice for copied Latin which is our primarily use case.
    let mut letterforms: HashMap<char, Vec<LetterformGroup>> = Default::default();
    for (path, font) in fonts.iter() {
        let upem = font.head().unwrap().units_per_em();
        let uniform_scale = if upem != max_upem {
            max_upem as f64 / upem as f64
        } else {
            1.0
        };
        for c in test_chars.iter() {
            let letterform = Letterform::create(font, *c, uniform_scale);

            glyphs.entry(*c).or_default().push(letterform.0.clone());

            let groups = letterforms.entry(*c).or_default();
            let mut grouped = false;
            for group in groups.iter_mut() {
                if group.matches(&letterform, rules) {
                    if group.insert(path, letterform.clone()).is_some() {
                        panic!("Multiple definitions for {path:?} '{c}");
                    }
                    grouped = true;
                }
            }
            if !grouped {
                groups.push(LetterformGroup::new(path, letterform));
            }
        }
    }
    Ok(letterforms)
}

fn main() {
    let args = Args::parse();
    init_logging();

    let test_chars = args.test_chars();
    let raw_fonts = load_fonts(args.files.iter().map(Path::new))
        .unwrap_or_else(|e| panic!("Unable to load fonts {e}"));

    let letterforms = create_grouped_letterforms(args.rules(), &test_chars, &raw_fonts).unwrap();

    log_groups(&test_chars, &letterforms);
    dump_stuff(&args, &letterforms);

    // Did we find sets of fonts that share glyphs?
    let mut share_counts: HashMap<BTreeSet<&Path>, usize> = Default::default();
    for groups in letterforms.values() {
        for group in groups {
            // It's really much more interesting when the group has multiple things in it
            if group.letterforms.len() < 2 {
                continue;
            }
            let key = group
                .letterforms
                .keys()
                .copied()
                .collect::<BTreeSet<&Path>>();
            let v = share_counts.entry(key).or_default();
            *v += 1;
        }
    }

    let limit = (test_chars.len() as f64 * args.match_pct / 100.0).ceil() as usize;
    println!(
        "Showing groups where at least {limit}/{} glyphs match\n\nGroup, Score",
        test_chars.len()
    );
    for (paths, score) in share_counts {
        if score >= limit {
            println!("{paths:?}, {score}/{}", test_chars.len());
        }
    }
}
