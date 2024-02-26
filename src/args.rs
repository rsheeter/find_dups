use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
    str::FromStr,
};

use clap::{command, Parser};

use crate::about_the_same::RulesOfSimilarity;

/// Reduced https://github.com/googlefonts/glyphsets/blob/main/Lib/glyphsets/definitions/nam/GF_Latin_Core.nam
const DEFAULT_TEST_STRING: &str = r#"abcdefghijklmnopqrstuvwxyz \
    ABCDEFGHIJKLMNOPQRSTUVWXYZ \
    1234567890\
    !?#$%&'()*+,-./:;<=>[\]^_,{|}"#;

const DEFAULT_WORKING_DIR: &str = "build";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// How near the nearest point must be to count as the same when comparing letterforms.
    ///
    /// Relative to 1000 upem. Default seems shockingly high but reflects actual observed results
    ///
    /// Even very visually similar families have diffs up to 2.5 or so.
    #[arg(long)]
    #[clap(default_value_t = 2.0)]
    pub equivalence: f64,

    /// If the sum of squared distance to nearest for all points exceeds budget consider the letterforms
    /// different. Relative to 1000 upem.
    #[arg(long)]
    #[clap(default_value_t = 100.0)]
    pub budget: f64,

    /// If any nearest test point is further apart than this consider the letterforms different
    #[arg(long)]
    #[clap(default_value_t = 25.0)]
    pub error: f64,

    /// If this percentage of the unique characters in --test-string match consider font(s) to match
    #[arg(long)]
    #[clap(default_value_t = 80.0)]
    pub match_pct: f64,

    /// Compare these characters to detect duplication
    #[arg(long)]
    #[clap(default_value_t = DEFAULT_TEST_STRING.to_string())]
    test_string: String,

    /// Use .nam file as source of test string. If set, overrides --test-string.
    ///
    /// E.g. --test-nam ../glyphsets/Lib/glyphsets/definitions/nam/GF_Latin_Core.nam
    #[arg(long)]
    test_nam: Option<String>,

    /// If set, for each unique character in --test-string write an svg showing variants
    #[arg(long)]
    pub dump_glyphs: bool,

    /// If set, write down the sets of files and common glyphs
    #[arg(long)]
    pub dump_groups: bool,

    /// Where to read/write temp files. Retention can accelerate repeat executions.
    #[arg(long)]
    #[clap(default_value_t = DEFAULT_WORKING_DIR.to_string())]
    pub working_dir: String,

    /// Path to repository containing subdirectories with font families.
    ///
    /// E.g. clone https://github.com/google/fonts to sibling dir "fonts" then
    /// pass --google-fonts ../fonts
    #[arg(long)]
    google_fonts: Option<String>,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    files: Vec<String>,
}

fn parse_nam_line(line: &str) -> Option<char> {
    let raw_codepoint = if let Some(cut) = line.find('#') {
        &line[..cut]
    } else {
        line
    }
    .trim();
    if raw_codepoint.is_empty() {
        return None;
    }
    if !raw_codepoint.starts_with("0x") {
        log::warn!("Invalid nam line: {line}");
        return None;
    }
    let l = if let Some(cut) = raw_codepoint.find(|c: char| c.is_ascii_whitespace()) {
        &raw_codepoint[2..cut]
    } else {
        &raw_codepoint[2..]
    };
    let codepoint = u32::from_str_radix(l, 16).expect("Bad nam");
    Some(char::from_u32(codepoint).expect("Bad codepoint"))
}

impl Args {
    pub fn rules(&self) -> RulesOfSimilarity {
        RulesOfSimilarity {
            equivalence: self.equivalence,
            budget: self.budget,
            error: self.error,
        }
    }

    // Returns unique, sorted, test characters
    pub fn test_chars(&self) -> Vec<char> {
        let mut test_chars = if let Some(test_nam) = &self.test_nam {
            io::BufReader::new(File::open(test_nam).expect("Unable to read .nam"))
                .lines()
                .filter_map(|l| parse_nam_line(l.as_deref().expect("To read nam lines")))
                .collect::<HashSet<_>>()
        } else {
            self.test_string.chars().collect::<HashSet<_>>()
        }
        .into_iter()
        .collect::<Vec<_>>();
        test_chars.sort();
        test_chars
    }

    pub fn font_files(&self) -> HashSet<PathBuf> {
        let mut files = HashSet::new();
        for file in self.files.iter() {
            let path = PathBuf::from_str(file).unwrap();
            if !path.is_file() {
                panic!("{path:?} is not a file");
            }
            files.insert(path);
        }
        if let Some(google_fonts) = &self.google_fonts {
            let mut google_fonts = google_fonts.to_owned();
            if !google_fonts.ends_with('/') {
                google_fonts.push('/');
            }
            google_fonts.push_str("**/METADATA.pb");
            for metadata_file in glob::glob(&google_fonts).unwrap() {
                let metadata_file = metadata_file.unwrap();
                let font_dir = metadata_file.parent().unwrap().to_str().unwrap().to_owned();
                let font_pattern = font_dir + "/*.[ot]tf";

                let mut font_files: Vec<_> = glob::glob(&font_pattern)
                    .unwrap()
                    .filter_map(|f| {
                        let f = f.unwrap();
                        if f.file_name().unwrap().to_str().unwrap().contains("-Italic") {
                            return None;
                        }
                        Some(f)
                    })
                    .collect();
                if font_files.len() == 1 {
                    // most VFs should take this path: max 2 files and -Italic was eliminated
                    let exemplar = font_files.pop().unwrap();
                    log::debug!("Picked {:?} as exemplar", exemplar);
                    files.insert(exemplar);
                } else if let Some(exemplar) = font_files.into_iter().find(|f| {
                    f.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .contains("-Regular")
                }) {
                    log::debug!("Picked {:?} as exemplar", exemplar);
                    files.insert(exemplar);
                } else {
                    log::warn!("Unable to identify an exemplar from {font_pattern}");
                }
            }
        }
        files
    }
}

#[cfg(test)]
mod tests {
    use crate::args::parse_nam_line;

    #[test]
    fn parse_nam_lines() {
        assert_eq!(
            vec![None, None, None, Some('a'), Some('B')],
            vec![
                parse_nam_line(""),
                parse_nam_line("\t#duck"),
                parse_nam_line("00A0"),
                parse_nam_line("0x61"),
                parse_nam_line("0x0042 DESC # mallard"),
            ]
        )
    }
}
