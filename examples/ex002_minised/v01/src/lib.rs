// Library crate

use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;

fn eprint_info(s: &str) {
    let info = format!("---- INFO: {}", s);
    eprintln!("{info}");
}

#[derive(Debug, PartialEq)]
pub struct Config {
    in_place: bool,
    pub script: String,
    file_path: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough command-line arguments");
        }
        if args.len() > 4 {
            return Err("Too many command-line arguments");
        }

        dbg!(&args);

        let (in_place, script, file_path) =
            if args.len() == 2 {
                // Format:  cargo run -- <script> < <file_path>
                // Example: cargo run -- 's/Monday/Tuesday/g' < ../data/mondays.csv
                // Note that STDIN will be read if a file path is not provided as an argument.
                (false, args[1].clone(), None)
            } else if args.len() == 3 {
                // Format:  cargo run -- <script> <file_path>
                // Example: cargo run -- 's/Monday/Tuesday/g' ../data/mondays.csv
                (false, args[1].clone(), Some(args[2].clone()))
            } else {
                // Format:  cargo run -- -i <script> <file_path>
                // Example: cargo run -- -i 's/Monday/Tuesday/g' ../data/mondays.csv
                if args[1] != "-i" {
                    return Err("The first command-line argument is not valid, expected `-i`");
                }
                (true, args[2].clone(), Some(args[3].clone()))
            };

        Ok(Config {
            in_place,
            script,
            file_path,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Flag {
    Global,
    Number(usize),
}

fn parse_flag(flag: &str) -> Result<Flag, &'static str> {
    let error_msg = "Flag must be 'g' or an integer greater than 0";
    if flag == "g" {
        Ok(Flag::Global)
    } else {
        let number: usize = match flag.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err(error_msg),
        };
        // In the script 's/pattern/replacement/flag', if flag is not 'g' then it must be a
        // positive, non-zero integer `num` that specifies that the `num`-th instance of the
        // pattern in the line should be replaced. In the function that performs the replacements,
        // the `num` value is reduced by 1 to get a zero-indexed position. `num` cannot be zero
        // because this results in a negative index position, which is invalid.
        if number > 0 {
            Ok(Flag::Number(number))
        } else {
            Err(error_msg)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Substitution {
    pattern: String,
    replacement: String,
    flag: Flag,
}

impl Substitution {
    pub fn build(script: &String) -> Result<Substitution, &'static str> {
        let split: Vec<&str> = script.split("/").collect();
        dbg!(&split);

        if split.len() != 4 {
            return Err("Invalid script format, expected 's/pattern/replacement/flag'");
        }

        let command_name = split[0];
        let pattern = String::from(split[1]);
        let replacement = String::from(split[2]);
        let flag = split[3];

        if command_name != "s" {
            return Err("Invalid command name, expected \"s\"");
        }

        let flag = parse_flag(flag)?;

        Ok(Substitution {
            pattern,
            replacement,
            flag,
        })
    }
}

fn replace_matches_globally(
    contents: &str,
    sub: &Substitution
) -> String {
    contents.replace(sub.pattern.as_str(), sub.replacement.as_str())
}

fn replace_numberth_match_in_single_line(
    line: &str,
    sub: &Substitution,
    num: usize,
) -> String {
    let matches: Vec<_> = line.match_indices(sub.pattern.as_str()).collect();

    if matches.len() == 0 {
        return line.to_string();
    }

    // In the script 's/pattern/replacement/flag', if flag is a positive, non-zero integer
    // `num`, then that specifies that the `num`-th instance of the pattern in the line should
    // be replaced. Subtract 1 from `num` to get a zero-indexed position.
    let numberth_index = num - 1;

    let (match_index_start, _pattern) = match matches.get(numberth_index) {
        Some(match_tuple) => match_tuple,
        None => return line.to_string(),
    };

    // Find the start and stop index of the `num`-th match of the pattern in the line.
    // Chop off the left side of the string (to left of the pattern) and the right side
    // of the string (to the right of the pattern). Insert replacement in between the
    // left side and right side and re-attach all three parts into a single String.
    let match_index_stop = match_index_start + sub.pattern.as_str().len();
    let (line_left_side, _) = line.split_at(match_index_start.clone());
    let (_, line_right_side) = line.split_at(match_index_stop.clone());
    let repl = sub.replacement.as_str();

    format!("{line_left_side}{repl}{line_right_side}")
}

fn replace_numberth_matches(
    contents: &str,
    sub: &Substitution,
    num: usize
) -> String {
    let mut new_contents = String::new();
    for line in contents.lines() {
        let new_line = replace_numberth_match_in_single_line(line, sub, num);
        new_contents.push_str(new_line.as_str());
        new_contents.push_str("\n");
    }
    new_contents
}

pub fn run(
    config: Config,
    sub: Substitution,
) -> Result<(), Box<dyn Error>> {
    dbg!(&config);
    dbg!(&sub);

    let contents = match &config.file_path {
        None => {
            // Read contents from STDIN if file path is not provided. If STDIN is not provided
            // via either a pipe '|' or a redirect '<' then the program will wait for the user
            // to enter STDIN manually. The `read_to_string` function will read all bytes until
            // EOF. If entering STDIN manually, press Control-D at the beginning of a new line
            // (on a Mac) to signal EOF.
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        },
        Some(file_path) => fs::read_to_string(file_path)?,
    };
    eprint_info("Original contents");
    eprintln!("{contents}");

    let new_contents: String = match &sub.flag {
        Flag::Global => {
            eprint_info("Replacing matches globally...");
            replace_matches_globally(&contents, &sub)
        }
        Flag::Number(num) => {
            let num = num.clone();
            eprint_info(&format!("Replacing the {}th match...", num));
            replace_numberth_matches(&contents, &sub, num)
        }
    };

    if config.in_place {
        match &config.file_path {
            None => {
                eprint_info("New contents");
                println!("{new_contents}");
            },
            Some(file_path) => {
                eprint_info("Writing new contents to file in-place");
                fs::write(file_path, new_contents)?;
            }
        }

    } else {
        eprint_info("New contents");
        println!("{new_contents}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_without_in_place_option_works() {
        let args: Vec<String> = vec![
            String::from("target/debug/ex002_minised_v01"),
            String::from("s/Monday/Friday/g"),
            String::from("../data/mondays.csv"),
        ];
        let config = Config::build(&args).unwrap();
        let config_exp = Config {
            in_place: false,
            script: String::from("s/Monday/Friday/g"),
            file_path: Some(String::from("../data/mondays.csv")),
        };
        assert_eq!(config, config_exp);
    }

    #[test]
    fn config_build_with_in_place_option_works() {
        let args: Vec<String> = vec![
            String::from("target/debug/ex002_minised_v01"),
            String::from("-i"),
            String::from("s/Monday/Friday/g"),
            String::from("../data/mondays.csv"),
        ];
        let config = Config::build(&args).unwrap();
        let config_exp = Config {
            in_place: true,
            script: String::from("s/Monday/Friday/g"),
            file_path: Some(String::from("../data/mondays.csv")),
        };
        assert_eq!(config, config_exp);
    }

    #[test]
    #[should_panic]
    fn config_build_with_too_few_arguments_panics() {
        let args: Vec<String> = vec![
            String::from("target/debug/ex002_minised_v01"),
        ];
        let _config = Config::build(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn config_build_with_too_many_arguments_panics() {
        let args: Vec<String> = vec![
            String::from("target/debug/ex002_minised_v01"),
            String::from("-i"),
            String::from("s/Monday/Friday/g"),
            String::from("../data/mondays.csv"),
            String::from("--extra-argument"),
        ];
        let _config = Config::build(&args).unwrap();
    }

    #[test]
    fn parse_flag_global_works() {
        let result = parse_flag("g").unwrap();
        assert_eq!(result, Flag::Global);
    }

    #[test]
    fn parse_flag_integer_works() {
        let result = parse_flag("5").unwrap();
        assert_eq!(result, Flag::Number(5));
    }

    #[test]
    #[should_panic]
    fn parse_flag_returns_error_for_invalid_input_z() {
        let _result = parse_flag("z").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_flag_returns_error_for_invalid_input_0() {
        let _result = parse_flag("0").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_flag_returns_error_for_invalid_input_float() {
        let _result = parse_flag("3.5").unwrap();
    }

    #[test]
    fn substitution_build_works_with_g_flag() {
        let script = String::from("s/Monday/Friday/g");
        let sub = Substitution::build(&script).unwrap();
        let sub_exp = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Global,
        };
        assert_eq!(sub, sub_exp);
    }

    #[test]
    fn substitution_build_works_with_integer_flag() {
        let script = String::from("s/Monday/Friday/3");
        let sub = Substitution::build(&script).unwrap();
        let sub_exp = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Number(3),
        };
        assert_eq!(sub, sub_exp);
    }

    #[test]
    #[should_panic]
    fn substitution_build_with_length_too_short_panics() {
        let script = String::from("s/Monday/Friday");
        let _sub = Substitution::build(&script).unwrap();
    }

    #[test]
    #[should_panic]
    fn substitution_build_with_length_too_long_panics() {
        let script = String::from("s/Monday/Friday/g/extra");
        let _sub = Substitution::build(&script).unwrap();
    }

    #[test]
    #[should_panic]
    fn substitution_build_with_invalid_command_name_panics() {
        let script = String::from("a/Monday/Friday/g");
        let _sub = Substitution::build(&script).unwrap();
    }

    #[test]
    #[should_panic]
    fn substitution_build_with_propagated_parse_flag_error_panics() {
        let script = String::from("s/Monday/Friday/z");
        let _sub = Substitution::build(&script).unwrap();
    }

    #[test]
    fn replace_matches_globally_works() {
        let contents = "\
column_1,column_2,column_3,column_4,column_5
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
";
        let sub = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Global,
        };
        let s = replace_matches_globally(contents, &sub);
        let s_exp = "\
column_1,column_2,column_3,column_4,column_5
Friday,Friday,Friday,Friday,Friday
Friday,Friday,Friday,Friday,Friday
Friday,Friday,Friday,Friday,Friday
";
        assert_eq!(s, s_exp);
    }

    #[test]
    fn replace_numberth_match_in_single_line_works_example_1() {
        let line = "\
column_1,column_2,column_3,column_4,column_5";
        let num = 2;
        let sub = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Number(num),
        };
        let s = replace_numberth_match_in_single_line(line, &sub, num);
        let s_exp = "\
column_1,column_2,column_3,column_4,column_5";
        assert_eq!(s, s_exp);
    }

    #[test]
    fn replace_numberth_match_in_single_line_works_example_2() {
        let line = "\
Monday,Monday,Monday,Monday,Monday";
        let num = 2;
        let sub = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Number(num),
        };
        let s = replace_numberth_match_in_single_line(line, &sub, num);
        let s_exp = "\
Monday,Friday,Monday,Monday,Monday";
        assert_eq!(s, s_exp);
    }

    #[test]
    fn replace_numberth_match_in_single_line_works_example_3() {
        let line = "\
Monday,Monday,Monday,Monday,Monday";
        let num = 8;
        let sub = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Number(num),
        };
        let s = replace_numberth_match_in_single_line(line, &sub, num);
        let s_exp = "\
Monday,Monday,Monday,Monday,Monday";
        assert_eq!(s, s_exp);
    }

    #[test]
    fn replace_numberth_matches_works_example_1() {
        let contents = "\
column_1,column_2,column_3,column_4,column_5
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
";
        let num = 2;
        let sub = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Number(num),
        };
        let s = replace_numberth_matches(contents, &sub, num);
        let s_exp = "\
column_1,column_2,column_3,column_4,column_5
Monday,Friday,Monday,Monday,Monday
Monday,Friday,Monday,Monday,Monday
Monday,Friday,Monday,Monday,Monday
";
        assert_eq!(s, s_exp);
    }

    #[test]
    fn replace_numberth_matches_works_example_2() {
        let contents = "\
column_1,column_2,column_3,column_4,column_5
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
";
        let num = 8;
        let sub = Substitution {
            pattern: String::from("Monday"),
            replacement: String::from("Friday"),
            flag: Flag::Number(num),
        };
        let s = replace_numberth_matches(contents, &sub, num);
        let s_exp = "\
column_1,column_2,column_3,column_4,column_5
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
Monday,Monday,Monday,Monday,Monday
";
        assert_eq!(s, s_exp);
    }
}
