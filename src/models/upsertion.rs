use diesel::result::Error as DbError;

trait TopLevelUpsert {
    fn upsert<'a>(name: &'a str) -> Result<i64, DbError>;
}
