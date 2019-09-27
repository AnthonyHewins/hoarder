#[derive(Queryable)]
pub struct Program {
    pub id: i32,
    pub publisher_id: i32,

    pub name: String,
    pub version: String
}

