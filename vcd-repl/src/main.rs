use std::path::PathBuf;
use structopt::StructOpt;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(StructOpt, Debug)]
pub struct Opts {
    file: PathBuf,
}

pub fn main() {
    let opts = Opts::from_args();
    println!("{:?}", opts);

    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
