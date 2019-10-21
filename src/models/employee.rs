#[derive(Queryable, Serialize, Deserialize)]
pub struct Employee {
    pub id: i64,
    pub name: String
}

