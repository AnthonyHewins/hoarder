#[derive(Queryable, Serialize, Deserialize)]
pub struct Publisher {
    pub id: i64,
    pub name: String
}
