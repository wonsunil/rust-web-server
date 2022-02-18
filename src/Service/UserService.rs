use std::collections::HashMap;
use mysql::Pool;
use mysql::from_row;
use mysql::prelude::Queryable;

use crate::service::Service;
use crate::db::connect::DataBaseAccess;

pub struct UserService{
    pool: Pool,
    service: Service::Service
}

impl UserService{
    pub fn query<S>(&self, sql: &str, params: HashMap<&str, S>) -> Vec<String>
    where
        S: Into<String>
    {
        let mut sql: String = sql.into();
        let pool = &self.pool;
        
        params.into_iter().for_each(|(key, value)| {
            let mut parameter = ":".to_owned();
            parameter.push_str(key);

            sql = sql.replace(&parameter, &("'".to_owned() + &value.into() + "'"));
        });

        let mut result: Vec<String> = Vec::new();

        pool.get_conn().unwrap().query_iter(sql).unwrap().for_each(|row| {
            result.push(from_row(row.unwrap()));
        });

        result
    }

    pub fn insert_user<S>(&mut self, user_info: HashMap<&str, S>) -> bool
    where
        S: Into<String>
    {
        match self.pool.get_conn() {
            Ok(mut conn) => {
                let query = self.service.set_sql_parameter("insert into user_info(id, password) values(:id, :password)", user_info);

                match conn.query_iter(query) {
                    Ok(_) => {
                        return true;
                    },
                    _ => {
                        return false;
                    }
                }
            },
            _ => {
                return false;
            }
        }
    }
}

pub fn new() -> UserService {
    let pool = DataBaseAccess::new().pool;

    UserService{
        pool: pool,
        service: Service::new()
    }
}