use std::env;
use std::fs;

mod lib;

fn main() {
    // let args:Vec<String> = env::args().collect();

    lib::parse_config(env::args());

    
    // match args.len() {
    //     1 => {
    //         println!("Usage of minigrep: {} <query> <path to file>", &args[0]);
    //         println!("missing <query> and <path to file>");
    //         std::process::exit(1);
    //     },
    //     2 => {
    //         println!("Usage of minigrep: {} {} <path to file>", &args[0], &args[1]);
    //         println!("                   {} {} {} [error] you're missing the path to filename", " ".repeat((&args[0]).len()), " ".repeat((&args[1]).len()), "^".repeat(13));
    //         println!("                   {} {} [info] this is the query (what you're looking for)"," ".repeat((&args[0]).len()), "^".repeat((&args[1]).len()));
    //         std::process::exit(1);
    //     }
    //     _ => (),
    // }

    // let query = &args[1];
    // let filepath = &args[2];

    // let contents = fs::read_to_string(filepath)
    //     .expect("[error] something went wrong read");

}


// 1. If no variable provided just print usage and exit
// 2. If one variable provided (query) -> print usage with first variable and helper msg
// -helper consist of two lines, error and info
// 3. If two variables provided run the program, if error print it
// 4. If three or more variables are provided print usage, mark from 3rd onward and write helper

// check for giberish
//      show error - even if it's without a side effect it may still be used in side effectuful way

// if query missing
//      show usage
// if filepath missing
//      show error

// if query_arg == FIRST_ARGUMENT.to_string() && filepath_arg == SECOND_ARGUMENT.to_string() {

// }