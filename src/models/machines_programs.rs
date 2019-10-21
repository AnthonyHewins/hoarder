#[derive(Queryable, Serialize, Deserialize)]
pub struct MachinesPrograms {
    pub id: i64,
    pub machine_id: i64,
    pub program_id: i64,
    pub sw_report_id: Option<i64>,

    pub path: Option<String>,
}
