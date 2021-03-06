/// si - string interpolator
/// author: David Clevenger
///
/// Interpolate stdin with environment variables or variable definitions
/// from a file and send to stdout
///
/// Use placeholder guards "${" and "}" around variable names that will be
/// looked up in either environment variables or a definitions file. Keys are
/// always *case-insensitive*.
///
/// e.g. city = CITY = cItY
///
/// si will succeed except for a few cases:
/// 1. placeholder guards are mismatched or incorrect
/// 2. -e (error) flag has been specified but a variable defintion was not found
///
/// File Formatting:
/// text files use the convention of a variable name, an equals sign (=), and a variables
/// defintion. for variable names or definitions with an embedded equals sign, use two equal
/// signs (==) which will be treated as a single equals within
///
/// example:
///
/// city=newyork
/// os=linux
/// equation=y == mx + b
///
/// equation will become "y = mx + b"
///
///
/// json files allow for a richer formatting -- embedded maps allow keys to be strung together
///
/// Example
///
/// {
///     "city": "newyork",
///     "os": "linux",
///     "equation": "y = mx + b"
///     "ocean": {
///         "creature": "whale",
///     }
/// }
///
/// "${ocean.creature}" will resolve to "whale"
/// 
///
/// Usage: 
/// $ si [-v] [-e] [-f <variables file>]
/// -v : acknowledge found and not found variables to stderr
/// -e : terminate with error if a variable is not found
/// -f <file> : specify a file with variable defintions (text or json)
///
/// use environment variables
/// $ cat raw.txt | si > processed.txt
///
/// use a file with variable definitions
/// $ cat raw.txt | si -f defs.json > processed.txt
/// $ cat raw.txt | si -f defs.txt > processed.txt
///
/// use stdin and stdout
/// $ echo "hello ${name}" | si > processed.txt

use std::{
    io::{self, prelude::*, Read, Write, BufReader},
    collections::HashMap, 
    path::Path,
    fs::File
};

use clap::{Arg, App};

enum Mode {
    TextFile(String),
    Env
}

fn parse(mode: Mode) -> HashMap<String, String> {
    let mut mapping = HashMap::new();

    match mode {
        Mode::TextFile(p) => {
            let f = File::open(p).unwrap();
            for line in BufReader::new(f).lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(e) => panic!("{}", e)
                };

                let parts: Vec<&str> = line.split("=").collect();
                if parts.len() != 2 { continue; } // Ignore blank and comment lines
                mapping.insert(parts[0].to_string(), parts[1].to_string());
            }
        },
        Mode::Env => {
            mapping = std::env::vars().collect();
        },
    }

    return mapping;
}

fn interpolate(verbose: bool, strict: bool, mapping: HashMap<String, String>) {
    let buf = &mut String::new();
    match io::stdin().read_to_string(&mut *buf) {
        Ok(_sz) => (),
        Err(_) => panic!("No input provided"),
    };

    let mut travel: usize = 0;
    while let Some(begin) = buf[travel..].find("${") {
        let begin= travel + begin;
        let end = match buf[begin..].find("}") {
            Some(end) => begin + end, // indexing from location of opening placeholder, so add begin index
            None => {
                let lineno = buf[..travel].as_bytes().iter().filter(|&&c| c == b'\n').count();
                panic!("Mismatched placeholders on line {}", lineno);
            } 
        };
        
        let key = &buf[begin+2..end];
        match mapping.get_key_value(key) {
            Some((k, v)) => {
                if verbose { eprintln!("Matched key: {} => {}", k, v) }
                buf.replace_range(begin..end+1, v); // include placeholders in range ([begin, end])
                travel = begin + v.len();
             },
            None => {
                if strict { panic!("Could not resolve key: {}", key); }
                if verbose { eprintln!("Could not resolve key: {}", key) }
                travel = end + 1;
            }
        }
    }

    match io::stdout().lock().write(buf.as_bytes()) {
        Ok(_sz) => (),
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let matches = App::new("si")
        .version("0.1.1")
        .author("David Clevenger <dclevenger00@gmail.com>")
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("acknowledge found and not found results for variable resolution to stderr"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .value_name("FILE")
            .help("text file to process with variable definitions"))
        .arg(Arg::with_name("error")
            .short("e")
            .long("error")
            .help("terminate on not found results"))
        .get_matches();
    
    let strict = matches.is_present("error");
    let verbose = matches.is_present("verbose");
    let mode: Mode = match matches.value_of("file") {
        Some(p) => match Path::new(p).extension() {
            Some(s) => match s.to_ascii_lowercase().to_str() {
                Some("txt") => Mode::TextFile(s.to_string_lossy().to_string()),
                Some(_) => panic!("Only text (\"txt\") files are supported"),
                None => panic!("Path is not UTF-8 encoded")
            }
            None => panic!("Unable to detect file extension"),
        }
        None => Mode::Env
    };

    let mapping = parse(mode);
    interpolate(verbose, strict, mapping);
}
