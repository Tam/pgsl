pub trait ToSql {
	fn to_sql(&self) -> Option<String>;
}
