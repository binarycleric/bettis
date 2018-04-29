pub use super::set_command::SetCommand;
pub use super::select_command::SelectCommand;
pub use super::get_command::GetCommand;
pub use super::del_command::DelCommand;
pub use super::incr_command::IncrCommand;
pub use super::decr_command::DecrCommand;

const SELECT_COMMAND: &'static str = "select";
const SET_COMMAND: &'static str = "set";
const GET_COMMAND: &'static str = "get";
const DEL_COMMAND: &'static str = "del";
const INCR_COMMAND: &'static str = "incr";
const DECR_COMMAND: &'static str = "decr";

pub enum Available {
    Select,
    Set,
    Get,
    Incr,
    Decr,
    Del,
}

impl Available {
    pub fn from_str<'a>(string: &'a str) -> Self {
        match string {
            SELECT_COMMAND => Available::Select,
            SET_COMMAND => Available::Set,
            GET_COMMAND => Available::Get,
            DEL_COMMAND => Available::Del,
            INCR_COMMAND => Available::Incr,
            DECR_COMMAND => Available::Decr,
            _ => panic!("Nooo"),
        }
    }
}
