use diesel::prelude::*;
use serde::Serialize;

use crate::models::session::Session;

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Identifiable,
    Associations,
    PartialEq,
    Debug,
    Clone,
    Serialize,
)]
#[diesel(table_name = crate::schema::classes)]
#[diesel(belongs_to(Session))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub code: String,
    #[diesel(column_name = "group_id")]
    pub group: String,
    pub session_id: i32,
}
