#[derive(Queryable, Serialize, Deserialize)]
pub struct Domain {
    pub id: i64,
    pub name: String
}
