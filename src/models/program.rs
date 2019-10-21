#[derive(Queryable, Serialize, Deserialize)]
pub struct Program {
    pub id: i64,
    pub publisher_id: Option<i64>,
    pub name: String,
    pub version: Option<String>
}
