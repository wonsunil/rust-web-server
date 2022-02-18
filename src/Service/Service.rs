use std::collections::HashMap;

pub struct Service{}

impl Service{
    pub fn set_sql_parameter<S>(&mut self, sql: &str, params: HashMap<&str, S>) -> String
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
}

pub fn new() -> Service{
    Service{}
}