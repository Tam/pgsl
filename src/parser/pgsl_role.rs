use crate::to_sql::ToSql;

#[derive(Debug, Default)]
pub struct PGSLRole {
	pub name: String,
	pub options: Vec<String>,
}

impl ToSql for PGSLRole {
	fn to_sql(&self) -> Option<String> {
		// TODO: check the DB to see if the role already exists, in which case
		//   return none
		
		let mut sql = format!(
			"create role {name}",
			name = self.name
		);
		
		if !self.options.is_empty() {
			sql.push_str(" with ");
			sql.push_str(&self.options.join(", "));
		}
		
		sql.push_str(";");
		
		Some(sql)
	}
}
