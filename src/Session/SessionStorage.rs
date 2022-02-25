use std::collections::HashMap;

use crate::Session;

pub struct SessionStorage{
    pub session: Vec<Session>
}

impl SessionStorage{
    pub fn get(&mut self, id: String) -> Session {
        if self.session.len() == 0 {
            let session = Session::new();

            self.session.push(session.clone());

            return session;
        };

        let session = self.session.iter().find(|session| {
            if session.get_id() == id {
                return true;
            }else {
                return false;
            };
        });

        match session {
            Some(session) => {
                Session::new_session(session)
            },
            None => {
                let session = Session::new();

                self.session.push(session.clone());

                return session;
            }
        }
    }

    pub fn set_session(&mut self, session: &mut Session) {
        let id = session.get_id();

        let find_session = self.session.iter().find(|session| {
            println!("Session: {:?}", session);
            if session.get_id() == id {
                true
            }else {
                false
            }
        });

        println!("find session {:?}", find_session);
    }

    pub fn clone(&self) -> SessionStorage {
        SessionStorage{
            session: self.session.clone()
        }
    }
}

pub fn new() -> SessionStorage {
    SessionStorage{
        session: Vec::new()
    }
}