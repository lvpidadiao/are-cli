use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{Arg, App};

use redis::Client;

pub mod process;


fn main() {

    let matches = App::new("another redis client")
        .version("0.0.1")
        .author("trevorlink <trevorlink@protonmail.com")
        .about("more simple redis client")
        .arg(Arg::with_name("hostname")
            .short("h")
            .help("host name to connect")
            .default_value("192.168.209.57"))
        .arg(Arg::with_name("port")
            .short("p")
            .help("redis connect port")
            .default_value("6379"))
        .get_matches();


    let hostname = matches.value_of("hostname").unwrap();
    let port = matches.value_of("port").unwrap();

    let mut redis_addr = String::from("redis://");
    redis_addr.push_str(hostname);
    redis_addr.push_str(":");
    redis_addr.push_str(port);

    let cli = Client::open(redis_addr.as_str()).unwrap();
    let con = cli.get_connection().unwrap();


    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("are_cli>> ");
        match readline {
            Ok(line) => {
                if line.eq("exit") || line.eq("quit") {
                    break;
                }
                rl.add_history_entry(line.as_str());
                let rlike = process::RESPLikeCmd::new(&line, &con);
                rlike.do_redis()
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
}
