use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::pool;
use crate::pool::RedisPool;
use super::RESPLikeCmd;

const HISTORY: &str = "save.cmd";

const K:&str = "hello";

// no helper
pub struct EditLine {
    rpool: RedisPool,
    rl: Editor<()>,
    prompt: String,
}

impl EditLine {
    pub fn new(pool: RedisPool,promp: &str) -> Self {
        EditLine {
            rpool: pool,
            rl : Editor::<()>::new(),
            prompt: String::from(promp),
        }
    }

    pub fn run(mut self) {
        self.rl.load_history(HISTORY);
        loop {
            let readline = self.rl.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    if line.eq("exit") || line.eq("quit") {
                        break;
                    }
                    self.rl.add_history_entry(line.as_str());
                    let con = self.rpool.get_conn();
                    let redis_cmd = RESPLikeCmd::new(&line, &con);
                    let res = redis_cmd.do_redis();
                    match res {
                        Ok(rep) => {
                            println!("{}", rep);
                            self.rpool.give_back(con);
                        }
                        Err(e) => {
                            println!("Redis Error: {}", e.to_string());
                        }
                    }
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
        self.rl.save_history(HISTORY);
    }
}


#[cfg(test)]
mod test {
    struct A {
        pub a: i32,
    }

    impl A {
        pub fn new() -> Self{
            eprintln!("a new A init.");
            A {
                a: 1,
            }
        }
    }

    fn pass(a: A) {
        eprintln!("i got it {}", a.a);
    }

    #[test]
    fn test_A() {
        let a = A::new();
        pass(a);
    }
}