use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::replicas;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = replicas)]
pub struct Replica {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub port: i32,
    pub connected_at: NaiveDateTime,
    pub is_online: bool,
}