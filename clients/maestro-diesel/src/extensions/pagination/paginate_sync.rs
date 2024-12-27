use {
	super::{dtos::PaginatedResultDTO, paginate::Paginated},
	diesel::{query_dsl::methods::LoadQuery, PgConnection, QueryResult},
};

pub trait Paginate: Sized {
	fn paginate<'a, U>(self, page: i32, page_size: i32, conn: &mut PgConnection) -> QueryResult<PaginatedResultDTO<U>>
	where
		Paginated<Self>: LoadQuery<'a, PgConnection, (U, i64)>;
}

impl<T> Paginate for T {
	fn paginate<'a, U>(self, page: i32, page_size: i32, conn: &mut PgConnection) -> QueryResult<PaginatedResultDTO<U>>
	where
		Paginated<Self>: LoadQuery<'a, PgConnection, (U, i64)>,
	{
		use diesel::RunQueryDsl;
		let page = page as i64;
		let page_size = page_size as i64;
		let offset = (page - 1) * page_size;
		let paginated = Paginated { query: self, page_size, offset, page };
		let results = paginated.load::<(U, i64)>(conn)?;
		let total = results.first().map(|x| x.1).unwrap_or(0);
		let records = results.into_iter().map(|x| x.0).collect();
		let total_pages = (total as f64 / page_size as f64).ceil() as i64;
		Ok(PaginatedResultDTO::new(records, total_pages, page))
	}
}
