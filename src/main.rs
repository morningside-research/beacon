use clap::{ArgEnum, Parser};
use shellwords;
use std::io;
use std::io::Write;
use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Display, EnumString, IntoStaticStr)] // strum macros.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug, Parser)]
enum Action {
    /// create
    Create,
    /// create
    Append,
    /// create
    List,
    /// break
    Break,
}
impl Default for Action {
    fn default() -> Self {
        Action::List
    }
}

#[non_exhaustive]
#[derive(Display, EnumString, ArgEnum, Clone, Debug, Parser)]
enum Mode {
    /// Interactive Interpreter
    Repl,
    /// Command line
    Cmd { action: Action },
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Repl
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t=Mode::Repl)]
    mode: Mode,
}

fn enact(action: Action) {
    match action {
        Action::Append => println!("Appending"),
        Action::List => println!("Listing"),
        Action::Create => println!("creating"),
        Action::Break => println!("creating"),
    }
}

fn prompt_action() -> Action {
    // Print command prompt and get command
    print!("bkn> ");
    io::stdout().flush().unwrap();

    let mut input = String::new(); // Take user input (to be parsed as clap args)
    io::stdin().read_line(&mut input).unwrap();
    let spoofed_first_arg = "beacon ".to_owned()+input.as_str();
    let words = shellwords::split(spoofed_first_arg.as_str()).unwrap();
    // prepend spoof string
    Action::parse_from(words)
}

fn main() {
    let args = Args::parse();
    loop {
        match args.mode {
            Mode::Repl => {
                let action = prompt_action();
                if action == Action::Break {
                    break;
                } else {
                    enact(action);
                }

                continue;
            }
            Mode::Cmd { action } => {
                enact(action);
                break;
            }
        };
    }
}
