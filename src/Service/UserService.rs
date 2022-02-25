use std::collections::HashMap;

use crate::util::{ vector, map };
use crate::service::Service;
use crate::logger;

pub struct UserService{
    service: Service::Service
}

impl UserService{
    pub fn insertUser<S>(&mut self, user_info: HashMap<&str, S>) -> bool
    where
        S: Into<String> + std::fmt::Debug
    {
        let (mut logger, _) = logger::new();

        logger.log("      InsertUser[");

        let result = self.service.insert("user_info", vector!{"id", "password"}, user_info);

        logger.log("      ]");

        result
    }

    pub fn selectUserById(&mut self, id: &str) -> HashMap<String, String> {
        let (mut logger, _) = logger::new();
        
        logger.log("      SelectUserById[");

        let result = self.service.select("user_info", vector!{"id", "user_role"}, map!{"id" => id});

        logger.log("      ]");

        result
    }

    pub fn selectUserByIdAndPassword(&mut self, user_info: HashMap<&str, String>) -> HashMap<String, String> {
        let (mut logger, _) = logger::new();
        
        logger.log("      SelectUserByIdAndPassword[");

        let result = self.service.select("user_info", vector!{"id", "user_role"}, user_info);

        logger.log("      ]");

        result
    }
}

pub fn new() -> UserService {
    UserService{
        service: Service::new()
    }
}