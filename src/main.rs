extern crate minigrep;

use clap::App;

use minigrep::Config;

fn main() {
    let matches = App::new("minigrep")
        .version("0.2")
        .author("Tasos S. <soukoulis.tasos at gmail.com>")
        .about("Search and print pattern matches from an input file")
        .arg("<pattern>              'Sets the pattern'")
        .arg("<input>                'Sets the input file to use'")
        .arg("-v...                  'Sets the level of verbosity'")
        .arg("-s, --sensitive        'Sets case sensitive")
        .get_matches();

    if let Err(e) = Config::new(matches).run() {
        eprintln!("{}", e)
    }
    // Same as previous example...
}
