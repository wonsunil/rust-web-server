use std::collections::HashMap;

use crate::util::{ vector, map };
use crate::service::Service;
use crate::logger;

pub struct UserService{
    service: Service::Service
}

impl UserService{
    pub fn insert_user<S>(&mut self, user_info: HashMap<&str, S>) -> bool
    where
        S: Into<String>
    {
        let result = self.service.insert("user_info", vector!{"id", "password"}, user_info);

        println!("{}", result);

        result
    }

    pub fn selectUserById(&mut self, id: &str) -> HashMap<String, String> {
        let (mut logger, _) = logger::new();
        
        logger.log("   Service: UserService");
        logger.log("   Method: selectUserById");

        self.service.select("user_info", vector!{"id", "user_role"}, map!{"id" => id})
    }

    pub fn selectUserByIdAndPassword(&mut self, user_info: HashMap<&str, String>) -> HashMap<String, String> {
        let (mut logger, _) = logger::new();
        
        logger.log("   Service: UserService");
        logger.log("   Method: selectUserById");

        self.service.select("user_info", vector!{"id", "user_role"}, user_info)
    }
}

pub fn new() -> UserService {
    UserService{
        service: Service::new()
    }
}