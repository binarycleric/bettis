mod set;
mod select;
mod get;
mod del;
mod incr;
mod decr;
mod expire;

pub use self::set::Set;
pub use self::select::Select;
pub use self::get::Get;
pub use self::del::Del;
pub use self::incr::Incr;
pub use self::decr::Decr;
pub use self::expire::Expire;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";
const DEL_COMMAND: &'static str = "del";
const INCR_COMMAND: &'static str = "incr";
const DECR_COMMAND: &'static str = "decr";
const EXPIRE_COMMAND: &'static str = "expire";

#[derive(Debug)]
pub enum Available {
    Select,
    Set,
    Get,
    Incr,
    Decr,
    Del,
    Expire,
}

impl Available {
    pub fn from_str(string: String) -> Self {
        match string.to_lowercase().as_str() {
            SELECT_COMMAND => Available::Select,
            SET_COMMAND => Available::Set,
            GET_COMMAND => Available::Get,
            DEL_COMMAND => Available::Del,
            INCR_COMMAND => Available::Incr,
            DECR_COMMAND => Available::Decr,
            EXPIRE_COMMAND => Available::Expire,
            _ => panic!("Nooo"),
        }
    }
}
