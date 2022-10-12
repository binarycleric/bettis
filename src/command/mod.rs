extern crate resp;

mod commands;

use storage::Database;

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

trait Runnable {
    fn invoke(&self, database: &mut Database) -> Result<resp::Value, resp::Value>;

    fn hash_key(values: &Vec<resp::Value>) -> String {
        match values[0] {
            resp::Value::Bulk(ref hash_key) => hash_key.clone(),
            _ => panic!("Shouldn't get here"),
        }
    }

    fn single_value(values: &Vec<resp::Value>) -> resp::Value {
        values[1].clone()
    }

    fn ok_response() -> resp::Value {
        resp::Value::String("OK".to_string())
    }

    fn error_response() -> resp::Value {
        resp::Value::Error("1".to_string())
    }
}


#[derive(Debug)]
pub struct Command {
    command_name: Available,
    values: Vec<resp::Value>,
}

impl Command {
    pub fn new(incoming: resp::Value) -> Self {
        let (command_name, values) = Self::extract_values(incoming);

        Self {
            command_name: command_name,
            values: values,
        }
    }

    fn extract_values(incoming: resp::Value) -> (Available, Vec<resp::Value>) {
        if let resp::Value::Array(ref array) = incoming {
            let command_name: Available;
            let values: Vec<resp::Value>;

            if let resp::Value::Bulk(ref cname) = array[0] {
                command_name = Available::from_str(String::from(cname));
            } else {
                panic!("This shouldn't happen");
            }

            values = array.get(1..).unwrap().clone().to_vec();

            return (command_name, values);
        }
        panic!("This shouldn't happen");
    }

    pub fn run(&self, database: &mut Database) -> Result<resp::Value, resp::Value> {
        let values = self.values.clone().to_vec();

        match self.command_name {
            Available::Select => {
                let command = commands::Select::new(values);
                command.invoke(database)
            }
            Available::Set => {
                let command = commands::Set::new(values);
                command.invoke(database)
            }
            Available::Get => {
                let command = commands::Get::new(values);
                command.invoke(database)
            }
            Available::Del => {
                let command = commands::Del::new(values);
                command.invoke(database)
            }
            Available::Incr => {
                let command = commands::Incr::new(values);
                command.invoke(database)
            }
            Available::Decr => {
                let command = commands::Decr::new(values);
                command.invoke(database)
            }
            Available::Expire => {
                let command = commands::Expire::new(values);
                command.invoke(database)
            }
        }
    }
}
