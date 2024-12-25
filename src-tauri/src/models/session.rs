use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug, PartialEq, Clone, Serialize)]
#[diesel(table_name = crate::schema::session)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: i32,
    pub year: i32,
    #[diesel(column_name = "session_name")]
    pub name: String,
}
