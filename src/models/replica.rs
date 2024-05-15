//use diesel::prelude::*;
//use serde::{Deserialize, Serialize};

//#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
//#[table_name = "replicas"]
#[derive(Debug)]
pub struct Replica {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub port: i32,
    pub connected_at: chrono::NaiveDateTime,
    pub is_online: bool,
}