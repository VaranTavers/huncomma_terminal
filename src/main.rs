mod utils;

use std::io;
use std::fs;
use std::io::Read;

use logos::Logos;

use huncomma::detector::{NaiveDetector, PairDetector, NaiveForwardDetector, TypicalDetector};
use huncomma::model::{PlainTextToken, Mistake, NaiveSettings, PairSettings, TypicalSettings};
use huncomma::traits::Detector;

use clap::{App, Arg};

use utils::merge_mistakes;

fn detect_errors_from_stdin(detectors: &mut Vec<Box<dyn Detector>>, merge_results: bool) -> io::Result<Option<Vec<(usize, usize, Mistake)>>> {
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
        .arg(Arg::with_name("min_cert")
            .short("c")
            .long("min_certainty")
            .help("Sets the minimum certainty(%) that is required for an error to be shown.")
            .takes_value(true)
            .default_value("30"))
        .get_matches();

    let mut detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(NaiveDetector::new(NaiveSettings::new_from_string(fs::read_to_string("naive.csv")?))),
        Box::new(NaiveForwardDetector::new(NaiveSettings::new_from_string(fs::read_to_string("naive_forward.csv")?))),
        Box::new(PairDetector::new(PairSettings::new_from_string(fs::read_to_string("pair.csv")?))),
        Box::new(TypicalDetector::new(TypicalSettings::new_from_string(fs::read_to_string("typical.csv")?))),
    ];

    let merge_results = match matches.occurrences_of("no_merge") {
        0 => true,
        _ => false,
    };

    let mut errors: Vec<(usize, usize, Mistake)> = Vec::new();

    while let Some(mut err) = detect_errors_from_stdin(&mut detectors, merge_results)?{
        errors.append(&mut err);
    }

    let min_cert = matches.value_of("min_cert").unwrap().parse::<f64>().unwrap() / 100.0;

    for (r, c, mistake) in errors {
        if mistake.prob > min_cert {
            println!("ln: {}, col: {} potenciális vesszőhiba ({}%): {}", r, c, mistake.prob * 100.0, mistake.get_str());
        }
    }

    Ok(())
}
