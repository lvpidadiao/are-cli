use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::pool;

const K:&str = "hello";

// no helper
pub struct EditLine {
    rl: Editor<()>,
    prompt: String,
}

impl EditLine {
    pub fn new(promp: &str) -> Self {
        EditLine {
            rl : Editor::<()>::new(),
            prompt: String::from(promp),
        }
    }

    pub fn run(mut self) {
        loop {
            let readline = self.rl.readline(&self.prompt);

            match readline {
                Ok(line) => {
                    if line.eq("exit") || line.eq("quit") {
                        break;
                    }
                    self.rl.add_history_entry(line.as_str());
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C, quit.");
                    break
                },
                Err(err) => {
                    println!("Error: {}", err.to_string());
                    continue
                }
            }
        }
    }
}