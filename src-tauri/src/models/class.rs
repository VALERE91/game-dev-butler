use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug, Clone, Serialize)]
pub struct Class {
    pub id: i32,
    pub name: String,
    pub code: String,
    #[diesel(column_name = "group_id")]
    pub group: String,
}
