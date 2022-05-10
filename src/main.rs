use std::iter::FromIterator;
use std::path::PathBuf;
use std::panic;

use backtrace::Backtrace;
use clap::{Arg, App};
use regex::Regex;

mod pcre2_regex;

fn main() {
    let matches = App::new("PT")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("A path transformation utility")
    
        .arg(Arg::with_name("INPUT")
            .help("The path to transform")
            .index(1)
        )

        .arg(Arg::with_name("extension")
            .short("e")
            .long("extension")
            .value_name("EXTENSION")
            .help("Overwrites the extension of the input path (Applied after transforms and filename)")
            .takes_value(true)
        )

        .arg(Arg::with_name("filename")
            .short("f")
            .long("filename")
            .value_name("FILENAME")
            .help("Overwrites the filename of the input path (Applied after transforms)")
            .takes_value(true)
        )

        .arg(Arg::with_name("boundless")
            .short("b")
            .long("boundless")
            .help("Ignore word boundaries when applying transforms")
        )

        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Prints verbose error messages")
        )

        .arg(Arg::with_name("TRANSFORMS")
            .help("The transformations to apply to the path in the format from=>to")
            .index(2)
            .multiple(true)
        )

        .get_matches();

    let verbose = matches.is_present("verbose");
    if cfg!(not(debug_assertions)) {
        panic::set_hook(Box::new(move |info| {
            println!("error: {}", panic_message::panic_info_message(info));
            
            if verbose {
                println!("{:?}", Backtrace::new());
            } else {
                println!("rerun with -v for verbose error messages");
            }
        }));
    }

    let mut path = PathBuf::from(matches.value_of("INPUT").expect("No input path specified"));
    
    if let Some(transformations) = matches.values_of("TRANSFORMS") {
        let transformations = transformations.into_iter().collect::<Vec<_>>();
    
        let mut path_str = path.into_os_string().into_string().unwrap();

        for transformation in transformations {
            let mut parts = transformation.split("=>");
            let from = parts.next().unwrap();
            let to = parts.next().unwrap();
            
            if matches.is_present("boundless") {
                path_str = path_str.replace(from, to);
            } else {
                path_str = Regex::new(&format!(r"\b{}\b", regex::escape(from))).unwrap().replace_all(&path_str, to).to_string();
            }
        }

        path = PathBuf::from(path_str);
    }
    
    if matches.is_present("filename") {
        path.set_file_name(matches.value_of("filename").unwrap());
    }

    if matches.is_present("extension") {
        path.set_extension(matches.value_of("extension").unwrap());
    }
    
    println!("{}", PathBuf::from_iter(path.iter()).display());

    /*unsafe {
        let regexp = pcre2_regex::Regex::new("([a-z]+)", 0).ok().unwrap();
        println!("{}", regexp.replace("Hello World!", r#"[\U$1]"#, 0, pcre2_regex::replace_options::SUBSTITUTE_EXTENDED | pcre2_regex::replace_options::SUBSTITUTE_GLOBAL));
    }*/
}
