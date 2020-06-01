use colored::*;
use std::env;

const USAGE_STRING: &str = "usage of minigrep:";
const FIRST_ARGUMENT: &str = "<query>";
const SECOND_ARGUMENT: &str = "<path to file>";

fn get_or_default(s: &[String], i: usize, def: &String) -> String {
    s.get(i).unwrap_or(def).to_string()
}

fn selector_msg(pad: usize, select_size: usize, msg: String) -> String {
    format!("{padding} {selector} {msg}",
        padding=" ".repeat(pad),
        selector="^".repeat(select_size),
        msg=msg)
}

fn build_usage(args: &[String]) -> Option<String> {
    let arg_len = args.len();

    let executable_path: &str = &args[0];

    let query_arg = get_or_default(args, 1, &(FIRST_ARGUMENT.to_string()));

    let filepath_arg: String = get_or_default(args, 2, &(SECOND_ARGUMENT.to_string()));

    let rest_arg = if arg_len > 2 {
        args[3..].join(" ")
    } else {
        "".to_string()
    };

    let usage_str = format!(
        "{} {} {} {} {}",
        USAGE_STRING,
        executable_path,
        query_arg,
        filepath_arg,
        rest_arg,
    );

    let mut helper_msg: String = String::new();


    let no_args = 1;
    let only_query = 2;
    let both_args = 3;

    match arg_len {
        no_args => helper_msg.push_str(""),
    }

    fn construct_msg(arg_len: usize, executable_path: &str, query_arg: &str, filepath_arg: &str) -> String {
        let no_args = 1;
        let only_query_arg = 2;
        let both_args = 3;

        if arg_len == no_args {
            return "".to_string();
        }

        let distance_to_query = USAGE_STRING.len() + executable_path.len() + 2;
        let distance_to_filepath = distance_to_query + query_arg.len();

       let helper_msg =  match arg_len {
            only_query_arg => {
                let pad_to_filepath = " ".repeat(distance_to_filepath);
                let selector_filepath = "^".repeat(filepath_arg.len());
                let msg = "missing filepath to be queried";

                let line_1 = format!("{pad} {selector} [error] {msg}",
                    pad = pad_to_filepath,
                    selector = selector_filepath,
                    msg = msg,
                );


                let pad_to_query = " ".repeat(distance_to_query);
                let selector_query = "^".repeat(query_arg.len());
                let msg = "this is the query (string you're looking for)";

                let line_2 = format!("{pad} {selector} [info] {msg}",
                    pad = pad_to_query,
                    selector = selector_query,
                    msg=msg,
                );

                format!("{}\n{}\n", line_1, line_2)
            },
            both_args:
            
        };
    };


    let helper_msg = match args.len() {
        1 => "".to_string(),
        2 => {
            let distance_to_query = USAGE_STRING.len() + executable_path.len() + 2;
            let distance_to_filepath = distance_to_query + query_arg.len() + 1;

            let pad_to_filepath = " ".repeat(distance_to_filepath);

            let filepath_selector = "^".repeat(filepath_arg.len());

            let errors_msg = format!(
                "\n{}{} [error] {}",
                pad_to_filepath,
                filepath_selector.bright_red(),
                "missing filepath to be queried"
            );

            let pad_to_query = " ".repeat(distance_to_query);
            let query_selector = "^".repeat(query_arg.len());

            let info_msg = format!(
                "\n{}{} [info] {}",
                pad_to_query,
                query_selector.yellow(),
                "this is the query (string you're looking for)"
            );

            errors_msg + info_msg.as_str()
        }
        3 => "".to_string(),
        _ => {
            let distance_to_query = USAGE_STRING.len() + executable_path.len() + 2;
            let distance_to_filepath = distance_to_query + query_arg.len() + 1;
            let distance_to_gibberish_start = distance_to_filepath + filepath_arg.len() + 1;

            let pad_to_gibberish = " ".repeat(distance_to_gibberish_start);

            let giberish_selector = "^".repeat(3);

            let errors_msg = format!(
                "\n{}{} [error] {}",
                pad_to_gibberish,
                giberish_selector.bright_red(),
                "from here on out is gibberish to minigrep"
            );

            errors_msg.to_owned()
        }
    };

    if arg_len > 3 {
        let gibberish_arg: &[String] = &args[3..];

        return Some(format!(
            "{} {} {} {} {}{}",
            USAGE_STRING,
            executable_path,
            query_arg,
            filepath_arg,
            gibberish_arg.join(" "),
            helper_msg,
        ));
    } else if arg_len < 3 {
        return Some(format!(
            "{} {} {} {}{}",
            USAGE_STRING, executable_path, query_arg, filepath_arg, helper_msg,
        ));
    }

    None
}

pub fn parse_config<'a, 'b>(args: env::Args) -> Result<(&'b str, &'b str), &'b str> {
    let args: Vec<String> = args.collect();

    let usage_str = build_usage(&args);

    if let Some(usage) = usage_str {
        println!("{}", usage);
    }

    Err("Bad message")
    // match args.len() {
    //     1 => Err(&format!(USAGE_STRING, &args[0])),
    //     2 => {
    //         println!(
    //             "Usage of minigrep: {} {} <path to file>",
    //             &args[0], &args[1]
    //         );
    //         println!(
    //             "                   {} {} {} [error] you're missing the path to filename",
    //             " ".repeat((&args[0]).len()),
    //             " ".repeat((&args[1]).len()),
    //             "^".repeat(13)
    //         );
    //         println!(
    //             "                   {} {} [info] this is the query (what you're looking for)",
    //             " ".repeat((&args[0]).len()),
    //             "^".repeat((&args[1]).len())
    //         );
    //         std::process::exit(1);
    //     }
    //     _ => (),
    // }
}
