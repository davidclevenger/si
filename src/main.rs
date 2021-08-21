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

use std::path::PathBuf;
use std::env;


fn interpolate_with_env(buf: &mut String) {
    
}

fn interpolate_with_file(buf: &mut String, filename: &PathBuf) {
    
}

fn main() {
    env::args().skip(1);
    


}
