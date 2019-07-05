use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use clap::{Arg, App};

use redis::Client;
use crate::pool::RedisPool;
use crate::process::line_edit::EditLine;

pub mod process;
pub mod pool;


fn main() {

    let matches = App::new("another redis client")
        .version("0.0.1")
        .author("trevorlink <trevorlink@protonmail.com")
        .about("more simple redis client")
        .arg(Arg::with_name("hostname")
            .short("h")
            .help("host name to connect")
            .default_value("127.0.0.1"))
        .arg(Arg::with_name("port")
            .short("p")
            .help("redis connect port")
            .default_value("6379"))
        .get_matches();


    let hostname = matches.value_of("hostname").unwrap();
    let port = matches.value_of("port").unwrap();

    let mut redis_addr = String::from("redis://");
    let prompt = format!("{}:{}", hostname, port);
    redis_addr.push_str(hostname);
    redis_addr.push_str(":");
    redis_addr.push_str(port);

    let cli = Client::open(redis_addr.as_str()).unwrap();

    let rpool = RedisPool::new(&redis_addr).expect("can't init redis pool");

    let el = EditLine::new(rpool, &format!("{}>> ", prompt)[0..]);

    el.run();
}
