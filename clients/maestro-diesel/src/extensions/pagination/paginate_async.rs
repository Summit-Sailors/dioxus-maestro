use {
	super::{dtos::PaginatedResultDTO, paginate::Paginated},
	crate::async_client::types::DbPoolAsync,
	diesel::QueryResult,
	diesel_async::{methods::LoadQuery, AsyncPgConnection},
};

pub trait PaginateAsync: Sized {
	fn paginate(self, page: i32, page_size: i32) -> Paginated<Self>;
}

impl<T> PaginateAsync for T {
	fn paginate(self, page: i32, page_size: i32) -> Paginated<Self> {
		Paginated { query: self, page_size: page_size as i64, page: page as i64, offset: (page - 1) as i64 * page_size as i64 }
	}
}

impl<T> Paginated<T> {
	pub async fn aload_and_count<'a, U>(self, conn: DbPoolAsync) -> QueryResult<(Vec<U>, i64)>
	where
		Self: LoadQuery<'a, AsyncPgConnection, (U, i64)> + 'a,
		U: std::marker::Send,
	{
		let results: Vec<(U, i64)> = {
			use diesel_async::RunQueryDsl;
			self.load::<(U, i64)>(&mut conn.get().await.expect("cant get conn from pool")).await?
		};
		let total = results.first().map(|x| x.1).unwrap_or(0);
		let records = results.into_iter().map(|x| x.0).collect();
		Ok((records, total))
	}

	pub async fn aload_paginated<'a, U>(self, conn: DbPoolAsync) -> QueryResult<PaginatedResultDTO<U>>
	where
		Self: LoadQuery<'a, AsyncPgConnection, (U, i64)> + 'a,
		U: std::marker::Send,
	{
		let page = self.page;
		let page_size = self.page_size;
		let (records, total): (Vec<U>, i64) = self.aload_and_count(conn).await?;
		let total_pages = (total as f64 / page_size as f64).ceil() as i64;
		Ok(PaginatedResultDTO::new(records, total_pages, page))
	}
}
