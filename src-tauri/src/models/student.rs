use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Clone, Serialize)]
#[diesel(table_name = crate::schema::student)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub code: String,
}
