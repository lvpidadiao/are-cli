
use redis::{Connection, ErrorKind, RedisResult};
pub mod line_edit;

pub struct RESPLikeCmd<'a>{
    pub cmd: String,
    conn: &'a Connection,
    key: Option<String>,
    args: Option<Vec<String>>,
}

impl<'a>  RESPLikeCmd<'a> {
    pub fn new(line: &str, conn: &'a Connection) -> RESPLikeCmd<'a> {

        let split_line: Vec<&str> = line.trim_end().split(' ').collect();
        if split_line.len() >= 3 {
            let mut args: Vec<String> = vec![];
            let mut i = 2;
            while i - 2 < split_line[2..].len() {
                args.push(String::from(split_line[i]));
                i+=1;
            }

            RESPLikeCmd{
                conn,
                cmd: String::from(split_line[0]),
                key: Some(String::from(split_line[1])),
                args: Some(args)
            }
        }else if split_line.len() == 2 {
            RESPLikeCmd{
                conn,
                cmd: String::from(split_line[0]),
                key: Some(String::from(split_line[1])),
                args: None,
            }
        }else {
            RESPLikeCmd{
                conn,
                cmd: String::from(split_line[0]),
                key: None,
                args: None,
            }
        }
    }

    pub fn do_redis(self) -> RedisResult<String> {
        let mut rcmd = redis::cmd(&self.cmd);
        if let Some(k) = self.key {
            rcmd.arg(&k);
        }
        if let Some(v) = self.args {
            for ref vv in v {
                rcmd.arg(vv);
            }
        }
        rcmd.query::<String>(self.conn)
//        let result = rcmd.query::<String>(self.conn);
//        match result {
//            Ok(o) => {
//            println!("{}", o)
//            },
//            Err(e) => {
//                match e.kind() {
//                    ErrorKind::IoError => {
//                        if !self.conn.is_open() {
//
//                        }
//                    }
//                    _ => {
//                        println!("{}", e.to_string())
//                    }
//                }
//
//            }
//        }
    }

    pub fn assemble(self) {

    }

    pub fn transfer(self) {

    }

    pub fn get_response(self) {

    }

    pub fn output_to_term(self) {

    }
}