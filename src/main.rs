mod utils;

use std::io;
use std::io::Read;

use logos::Logos;

use huncomma::detector::{NaiveDetector, PairDetector, NaiveForwardDetector};
use huncomma::model::{PlainTextToken, Mistake, NaiveSettings, PairSettings};
use huncomma::traits::Detector;

use clap::{App, Arg};

use utils::merge_mistakes;

fn main_loop(detectors: &mut Vec<Box<dyn Detector>>, merge_results: bool) -> io::Result<Option<Vec<(usize, usize, Mistake)>>> {
    let mut errors = Vec::new();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Ok(None);
    }

    let tokens = PlainTextToken::lexer(buffer.as_str());

    if merge_results {
        let c_errors = detectors.iter_mut().map(|detector| detector.detect_errors(&mut tokens.clone())).collect();
        errors.append(&mut merge_mistakes(c_errors));
    } else {
        for detector in detectors.iter_mut() {
            let mut c_errors = detector.detect_errors(&mut tokens.clone());
            errors.append(&mut c_errors);
        }
    }

    Ok(Some(errors))
}

fn main() -> io::Result<()> {
    let matches = App::new("huncomma_terminal")
        .version("0.1")
        .author("Tasnádi Zoltán <tasnadi98@tutanota.com>")
        .about("Detects some potential mistakes regarding the usage of commas in Hungarian.")
        .arg(Arg::with_name("no_merge")
            .short("n")
            .long("no_merge")
            .help("Disables merging mistakes from different detectors. May make the program faster"))
        .get_matches();

    let mut detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(NaiveDetector::new(NaiveSettings::new_from_file("naive.csv"))),
        Box::new(NaiveForwardDetector::new(NaiveSettings::new_from_file("naive_forward.csv"))),
        Box::new(PairDetector::new(PairSettings::new_from_file("pair.csv"))),
    ];

    let merge_results = match matches.occurrences_of("no_merge") {
        0 => true,
        _ => false,
    };

    let mut errors: Vec<(usize, usize, Mistake)> = Vec::new();

    while let Some(mut err) = main_loop(&mut detectors, merge_results)?{
        errors.append(&mut err);
    }

    for (r, c, mistake) in errors {
        if mistake.prob > 0.30 {
            println!("ln: {}, col: {} potenciális vesszőhiba ({}%): {}", r, c, mistake.prob * 100.0, mistake.get_str());
        }
    }

    Ok(())
}
