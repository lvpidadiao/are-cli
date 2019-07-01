
use rustyline::Editor;

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

        }
    }
}