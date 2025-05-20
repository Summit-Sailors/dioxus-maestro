use {
	diesel::{
		pg::Pg,
		query_builder::{AstPass, Query, QueryFragment, QueryId},
		sql_types::BigInt,
		PgConnection, QueryResult,
	},
	std::fmt::Debug,
};

#[derive(Clone, Copy, Debug, QueryId)]
pub struct Paginated<T> {
	pub query: T,
	pub page_size: i64,
	pub offset: i64,
	pub page: i64,
}

impl<T> QueryFragment<Pg> for Paginated<T>
where
	T: QueryFragment<Pg>,
{
	fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
		out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
		self.query.walk_ast(out.reborrow())?;
		out.push_sql(") as paged_query_with LIMIT ");
		out.push_bind_param::<BigInt, _>(&self.page_size)?;
		out.push_sql(" OFFSET ");
		out.push_bind_param::<BigInt, _>(&self.offset)?;
		Ok(())
	}
}

impl<T: Query> Query for Paginated<T> {
	type SqlType = (T::SqlType, BigInt);
}

impl<T> diesel::RunQueryDsl<PgConnection> for Paginated<T> {}
