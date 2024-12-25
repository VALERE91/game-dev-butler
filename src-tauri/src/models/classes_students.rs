use diesel::prelude::*;
use serde::Serialize;

use crate::models::{class::Class, student::Student};

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, PartialEq, Clone, Serialize)]
#[diesel(belongs_to(Class))]
#[diesel(belongs_to(Student))]
#[diesel(table_name = crate::schema::classes_student)]
pub struct ClassesStudents {
    pub id: i32,
    pub class_id: i32,
    pub student_id: i32,
}
