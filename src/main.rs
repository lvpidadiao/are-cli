

use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub mod process;


fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("are_cli>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let rlike = process::RESPLikeCmd::new(&line);

                println!("Line :{}", line);
                println!("Cmd is :{}", rlike.Cmd);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("hello.txt").unwrap();
    println!("Hello, world!");
}
