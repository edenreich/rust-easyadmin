use diesel::sql_types::Datetime;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub updated_at: Datetime,
    pub created_at: Datetime,
}
