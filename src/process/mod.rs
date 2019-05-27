
pub struct RESPLikeCmd{
    pub Cmd: String,
    Key: Option<String>,
    Args: Option<Vec<String>>,
}

impl RESPLikeCmd {
    pub fn new(line : &str) -> RESPLikeCmd {
        let splitLine: Vec<&str> = line.split(' ').collect();
        if splitLine.len() >= 3 {
            let mut args: Vec<String> = vec![];
            let mut i = 2;
            while i < splitLine[2..].len() {
                args.push(String::from(splitLine[i]));
                i+=1;
            }

            RESPLikeCmd{
                Cmd: String::from(splitLine[0]),
                Key: Some(String::from(splitLine[1])),
                Args: Some(args)
            }
        }else if splitLine.len() == 2 {
            RESPLikeCmd{
                Cmd: String::from(splitLine[0]),
                Key: Some(String::from(splitLine[1])),
                Args: None,
            }
        }else {
            RESPLikeCmd{
                Cmd : String::from(splitLine[0]),
                Key: None,
                Args: None,
            }
        }
    }

    pub fn assemble(self) {

    }

    pub fn transfer(self) {

    }

    pub fn getResponse(self) {

    }

    pub fn outputToClient(self) {

    }
}