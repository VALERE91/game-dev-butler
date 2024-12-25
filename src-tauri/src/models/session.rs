use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Clone, Serialize)]
#[diesel(table_name = crate::schema::session)]
pub struct Session {
    pub id: i32,
    pub year: i32,
    #[diesel(column_name = "session_name")]
    pub name: String,
}
