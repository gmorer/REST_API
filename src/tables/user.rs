use diesel::dsl::sql;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

const TABLE_MODEL: &str = "CREATE TABLE users (
        id         SERIAL PRIMARY KEY,
        username   TEXT NOT NULL,
        password   TEXT NOT NULL
    )";

#[derive(From)]
pub enum Error {
	R2d2Error(r2d2::Error),
	TablesError(diesel::result::Error),
}

#[derive(Debug, Queryable)]
pub struct User {
	id: i32,
	username: String,
	password: String,
}

table! {
	users (id) {
		id -> Int4,
		username -> Text,
		password -> Text,
	}
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	pub username: &'a str,
	pub password: &'a str,
}

pub struct UserDb {
	pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserDb {
	pub fn create_tab(
		conn: PooledConnection<ConnectionManager<PgConnection>>,
	) -> Result<usize, diesel::result::Error> {
		sql::<()>(TABLE_MODEL).execute(&conn)
	}

	pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
		UserDb { pool }
	}

	pub fn get_all(&self) -> Result<String, Error> {
		// let mut res = Vec::new();
		// for row in self.pool.get()?.query("SELECT id username password FROM users", &[])? {
		// 	res.push(User {
		// 		id: row.get(0),
		// 		username: row.get(1),
		// 		password: row.get(2)
		// 	})
		// }
		// Ok(format!("{:?}", res))
		use self::users::dsl::*;
		let conn = self.pool.get()?;
		let res = users
			.limit(5)
			.load::<User>(&conn)
			.expect("Error loading posts");
		println!("{:?}", res);
		Ok("no implemented".to_string())
	}

	pub fn insert(&self, username: &str, password: &str) -> Result<User, Error> {
		let conn = self.pool.get()?;
		let new_user = NewUser { username, password };

		diesel::insert_into(users::table)
			.values(&new_user)
			.get_result(&conn)
			.map_err(|e| e.into())
	}
}
