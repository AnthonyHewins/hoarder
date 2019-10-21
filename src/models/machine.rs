#[derive(Queryable, Serialize, Deserialize)]
pub struct Machine {
    pub id: i64,
    pub domain_id: Option<i64>,
    pub employee_id: Option<i64>,
    pub host: String,
}
