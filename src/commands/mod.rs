mod set;

use types::QDataType;
use storage::Database;

pub use self::set::SetCommand;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";
const INCR_COMMAND: &'static str = "incr";
const DECR_COMMAND: &'static str = "decr";
const DEL_COMMAND: &'static str = "del";

#[derive(Debug, PartialEq)]
enum Available {
    Select,
    Set,
    Get,
    Incr,
    Decr,
    Del,
}

impl Available {
    fn from_str<'a>(command: &'a str) -> Result<Available, &'static str> {
        match command {
            SELECT_COMMAND => Ok(Available::Select),
            SET_COMMAND => Ok(Available::Set),
            GET_COMMAND => Ok(Available::Get),
            INCR_COMMAND => Ok(Available::Incr),
            DECR_COMMAND => Ok(Available::Decr),
            DEL_COMMAND => Ok(Available::Del),
            _ => Err("Invalid redis command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Available;

    #[test]
    fn it_parses_command_from_string() {
        assert_eq!(Ok(Available::Select), Available::from_str("select"));
        assert_eq!(Ok(Available::Set), Available::from_str("set"));
        assert_eq!(Ok(Available::Get), Available::from_str("get"));
    }
}
