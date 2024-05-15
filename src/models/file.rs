//use diesel::prelude::*;
//use serde::{Deserialize, Serialize};

//#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
//#[table_name = "files"]
#[derive(Debug)]
pub struct File {
    pub id: i32,
    pub filepath: String,
    pub size: i32,
    pub file_date: chrono::NaiveDateTime,
    pub replicas: String,
    pub to_delete: bool,
    pub updated_at: chrono::NaiveDateTime,
}