use std::collections::HashMap;
use mysql::{ Pool, from_row };
use mysql::prelude::Queryable;

use crate::db::connect::DataBaseAccess;
use crate::logger;

pub struct Service{
    pool: Pool
}

impl Service{
    fn set_sql_parameter<S>(&mut self, sql: &str, params: HashMap<&str, S>) -> String
    where
        S: Into<String>
    {
        let mut sql = String::from(sql);

        params.into_iter().for_each(|(key, value)| {
            let mut parameter = ":".to_owned();
            parameter += key;

            sql = sql.replace(&parameter, &("'".to_owned() + &value.into() + "'"));
        });

        sql
    }

    fn set_where_clause<S>(&mut self, params: &HashMap<&str, S>) -> String
    where
        S: Into<String> + std::fmt::Debug
    {
        let mut sql = String::from("");

        let length = params.len();
        let mut index = 0;

        params.keys().cloned().collect::<Vec<&str>>().iter().for_each(|key| {
            sql.push_str(key);
            sql.push_str(" = ");
            sql.push_str(&(":".to_owned() + key));

            index += 1;

            if index < length {
                sql.push_str(" and ");
            };
        });

        sql
    }

    pub fn query<S>(&mut self, sql: &str, params: HashMap<&str, S>) -> Vec<String>
    where
        S: Into<String>
    {
        let sql: String = self.set_sql_parameter(sql, params);
        let pool = &self.pool;

        let mut result: Vec<String> = Vec::new();

        pool.get_conn().unwrap().query_iter(sql).unwrap().for_each(|row| {
            result.push(from_row(row.unwrap()));
        });

        result
    }

    pub fn insert<S>(&self, table: &str, params: Vec<&str>, values: HashMap<&str, S>) -> bool
    where
        S: Into<String>
    {
        let query = format!("insert into {}({})", table, params.join(","));

        println!("{}", query);

        false
    }

    pub fn select<S>(&mut self, table: &str, params: Vec<&str>, values: HashMap<&str, S>) -> HashMap<String, String>
    where
        S: Into<String> + std::fmt::Debug
    {
        let mut sql = format!("select {} from {} where {}", params.join(", "), table, self.set_where_clause(&values));
        let (mut logger, _) = logger::new();
        let mut keys: Vec<String> = Vec::new();

        params.into_iter().for_each(|key| {
            keys.push(key.to_owned());
        });

        let mut result: HashMap<String, String> = HashMap::new();

        logger.log(&format!("   Query: {}", sql));

        sql = self.set_sql_parameter(&sql, values);

        self.pool.get_conn().unwrap().query_iter(sql).unwrap().for_each(|row| {
            let keys = keys.clone();
            let mut index = 0;

            keys.into_iter().for_each(|key| {
                result.insert(key, row.as_ref().unwrap().get(index).unwrap());

                index += 1;
            });
        });

        result
    }
}

pub fn new() -> Service{
    let pool = DataBaseAccess::new().pool;
    
    Service{
        pool: pool
    }
}