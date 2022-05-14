use std::iter::FromIterator;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::panic;

use backtrace::Backtrace;
use path_slash::PathExt;
use clap::{Arg, App};

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
            .long("ext")
            .value_name("EXTENSION")
            .help("Overwrites the extension of the input path (Applied after transforms and filename overwrite)")
            .takes_value(true)
        )

        .arg(Arg::with_name("filename")
            .short("f")
            .long("fname")
            .value_name("FILENAME")
            .help("Overwrites the filename of the input path (Applied after transforms)")
            .takes_value(true)
        )

        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Prints verbose error messages")
        )

        .arg(Arg::with_name("TRANSFORMS")
            .help("The transformations to apply to the path in the form from~to or from~~to (The first form only matches entire directory names, the second applies globally as a PCRE2 RegEx; the to value may use extended PCRE2 substitutions in either form)")
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
            if let Some((pattern,replacement)) = transformation.split_once("~~") {
                path_str = unsafe {pcre2_regex::Regex::new(pattern, pcre2_regex::compile_options::UTF).expect("Invalid pattern").replace(&mut path_str, replacement, 0, pcre2_regex::replace_options::SUBSTITUTE_EXTENDED | pcre2_regex::replace_options::SUBSTITUTE_GLOBAL)};
            }
            else if let Some((text,replacement)) = transformation.split_once("~") {
                let buf = PathBuf::from(path_str.clone());
                let filename = buf.file_name().unwrap_or(OsStr::new(""));
                path_str = unsafe {pcre2_regex::Regex::new(format!(r"(?<=^|\/){}(?=$|\/)",regex::escape(text)), pcre2_regex::compile_options::UTF).expect("Invalid pattern").replace(&mut path_str.clone(), replacement, 0, pcre2_regex::replace_options::SUBSTITUTE_EXTENDED | pcre2_regex::replace_options::SUBSTITUTE_GLOBAL)};
                let mut buf = PathBuf::from(path_str.clone());
                buf.set_file_name(filename);
                path_str = buf.into_os_string().into_string().unwrap();
            }
            else {
                panic!("Invalid transformation: {}", transformation);
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
    
    println!("{}", PathBuf::from_iter(path.iter()).to_slash().unwrap());
}
