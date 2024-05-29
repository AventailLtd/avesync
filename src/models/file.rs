use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::files;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = files)]
pub struct File {
    pub id: i32,
    pub filepath: String,
    pub size: i32,
    pub file_date: NaiveDateTime,
    pub replicas: String,
    pub to_delete: bool,
    pub updated_at: NaiveDateTime,
}