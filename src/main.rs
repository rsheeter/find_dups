use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs, io,
    path::{Path, PathBuf},
};

use clap::{command, Parser};
use kurbo::{Affine, BezPath, ParamCurve, ParamCurveNearest, PathEl, Point, Shape};
use skrifa::{instance::Size, raw::TableProvider, FontRef, MetadataProvider};
use thiserror::Error;
use write_fonts::pens::BezPathPen;

const DEFAULT_TEST_STRING: &str =
    r#"1234567890-=!@#$%^&*()_+qWeRtYuIoP[]|AsDfGhJkL:"zXcVbNm,.<>{}[]üøéåîÿçñè"#;

const DEFAULT_WORKING_DIR: &str = "build";

const NEAREST_EPSILON: f64 = 0.0000001;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// How near the nearest point must be to count as the same when comparing letterforms.
    ///
    /// Relative to 1000 upem. Default seems shockingly high but reflects actual observed results
    ///
    /// Even very visually similar families have diffs up to 2.5 or so.
    #[arg(long)]
    #[clap(default_value_t = 2.0)]
    equivalence: f64,

    /// If the sum of squared distance to nearest for all points exceeds budget consider the letterforms
    /// different. Relative to 1000 upem.
    #[arg(long)]
    #[clap(default_value_t = 100.0)]
    budget: f64,

    /// If any nearest test point is further apart than this consider the letterforms different
    #[arg(long)]
    #[clap(default_value_t = 25.0)]
    error: f64,

    /// If this percentage of the unique characters in --test-string match consider font(s) to match
    #[arg(long)]
    #[clap(default_value_t = 80.0)]
    match_pct: f64,

    /// Compare these characters to detect duplication
    #[arg(long)]
    #[clap(default_value_t = DEFAULT_TEST_STRING.to_string())]
    test_string: String,

    /// If set, for each unique character in --test-string write an svg showing variants
    #[arg(long)]
    dump_glyphs: bool,

    /// Where to read/write temp files. Retention can accelerate repeat executions.
    #[arg(long)]
    #[clap(default_value_t = DEFAULT_WORKING_DIR.to_string())]
    working_dir: String,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    files: Vec<String>,
}

impl Args {
    fn rules(&self) -> RulesOfSimilarity {
        RulesOfSimilarity {
            equivalence: self.equivalence,
            budget: self.budget,
            error: self.error,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RulesOfSimilarity {
    equivalence: f64,
    budget: f64,
    error: f64,
}

impl RulesOfSimilarity {
    fn for_upem(self, upem: u16) -> Self {
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
enum ApproximatelyEqualError {
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

trait AboutTheSame<T = Self> {
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
        .filter(|p| {
            if !p.is_file() {
                log::warn!("{p:?} is not a file");
                return false;
            }
            true
        })
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
        fs::write(working_dir.join(format!("{c}{suffix}.svg")), svg).unwrap();
    }
}

fn main() {
    let args = Args::parse();
    init_logging();

    let raw_fonts = load_fonts(args.files.iter().map(Path::new))
        .unwrap_or_else(|e| panic!("Unable to load fonts {e}"));

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
        log::warn!("Not much to do with no fonts specified");
        return;
    }

    // we will scale to the largest upem
    let max_upem = fonts
        .values()
        .map(|f| f.head().unwrap().units_per_em())
        .max()
        .unwrap();
    let mut test_chars = args
        .test_string
        .chars()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    test_chars.sort();
    let test_chars = test_chars;
    let mut glyphs: HashMap<char, Vec<BezPath>> = Default::default();

    // budget is based on 1000 upem; scale if necessary
    let rules = args.rules().for_upem(max_upem);
    log::info!("The rules are {rules:?}");

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

    if args.dump_glyphs {
        let working_dir = Path::new(&args.working_dir);
        if !working_dir.is_dir() {
            fs::create_dir(working_dir).unwrap();
        }
        println!("Dumping glyphs to {working_dir:?}");
        dump_glyphs(working_dir, &letterforms);
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
