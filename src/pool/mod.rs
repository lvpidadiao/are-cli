use redis::{Client, Connection};

use std::cell::RefCell;
use std::collections::linked_list::LinkedList;
use redis::RedisResult;

pub struct RedisPool {
    cli: Client,
    connvec: RefCell<Vec<Connection>>,
}

impl RedisPool {
    pub fn new(url: &str) -> RedisResult<Self> {
        let cli = Client::open(url)?;
        Ok(
            RedisPool {
                cli: cli,
                connvec: RefCell::new(Vec::new()),
            }
        )
    }

    fn get_conn(&self) -> Connection {
        let mut mconn = self.connvec.borrow_mut();

        if mconn.len() == 0 {
            match self.cli.get_connection() {
                Ok(c) => c,
                Err(e) => {
                    panic!(format!("Err: {}", e.to_string()));
                }
            }
        } else {
            mconn.pop().unwrap()
        }
    }

    fn ret_conn(&self, conn: Connection) {
        self.connvec.borrow_mut().push(conn);
    }
}