use std::fmt;
use std::collections::HashMap;
use rand::{ thread_rng, Rng, distributions::Alphanumeric };

use crate::util::json;

#[derive(Clone)]
pub struct Session{
    id: String,
    data: HashMap<String, String>
}

impl fmt::Debug for Session {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        println!("Session self data {:?}", self.data);

        write!(format, r#"Session [id: {}, data: {}]"#, self.id, json::stringify(self.data.clone()))
    }
}

impl Session{
    fn generate_session_id() -> String {
        thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
    }

    pub fn clone(&mut self) -> Session {
        Session{
            id: self.id.clone(),
            data: self.data.clone()
        }
    }

    pub fn get_id(&self) -> String {
        (*self.id).into()
    }

    pub fn get_data(&self) -> HashMap<String, String> {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: HashMap<String, String>) {
        for (key, value) in data {
            self.data.insert(key.into(), value.into());
        }

        println!("session set_data self data {:?}", self.data);
    }

    pub fn empty() -> Session {
        Session{
            id: "".into(),
            data: HashMap::new()
        }
    }

    pub fn new() -> Session {
        let session = Session{
            id: Session::generate_session_id(),
            data: HashMap::new()
        };

        session
    }

    pub fn new_session(session: &Session) -> Session {
        Session{
            id: session.get_id(),
            data: session.get_data()
        }
    }
}