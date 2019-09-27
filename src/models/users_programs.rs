use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct UsersPrograms {
    pub id: i32,
    pub machine_id: i32,
    pub program_id: i32,

    pub date_installed: NaiveDateTime,
    pub path: String
}
