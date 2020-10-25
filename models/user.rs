use diesel::{Queryable, sql_types::Datetime};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub updated_at: Datetime,
    pub created_at: Datetime,
}
