extern crate log;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[, \t\n\f]+")]
enum ZabbixKeyToken {
    #[regex("(\")*[0-9a-zA-Z./_-]+(\")*")]
    Ident,
    #[token("[")]
    Renc,
    #[token("]")]
    Lenc,
}

pub fn zabbix_key_to_zenoh(key: String) -> Option<String> {
    log::trace!("Parsing: {:?}", &key);
    let mut res = String::from("".to_string());
    let mut lex = ZabbixKeyToken::lexer(&key);
    loop {
        match lex.next() {
            Some(Ok(ZabbixKeyToken::Ident)) => {
                let mut val = (&lex.slice()).to_string();
                log::trace!("Got ident: {:?}", &val);
                if val.is_empty() {
                    val = "_".to_string();
                } else {
                    val = val.replace("/", "\\");
                }
                if val.chars().nth(0) == Some('\"') {
                    val = (&val[1..val.len() - 1]).to_string();
                }
                res = [res, "/".to_string(), val].join("");
            }
            Some(Ok(ZabbixKeyToken::Lenc)) => {
                break;
            }
            Some(Err(err)) => {
                log::warn!("Error converting Zabbix key {} = {}: {:?}", &key, &lex.slice(), err);
                return None;
            }
            None => break,
            Some(Ok(something)) => {
                log::trace!("Got something: {:?} {:?}", something, (&lex.slice()).to_string());
            }
        }
    }
    Some(res)
}
