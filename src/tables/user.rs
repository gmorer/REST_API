use postgres::{ Client, NoTls };
use r2d2_postgres::PostgresConnectionManager;
use r2d2::{ Pool };

#[derive(From)]
pub enum Error {
	R2d2Error(r2d2::Error),
	PostgressError(postgres::error::Error)
}

const TABLE_MODEL: &str = "CREATE TABLE users (
        id         SERIAL PRIMARY KEY,
        username   TEXT NOT NULL,
        password   TEXT NOT NULL
    )";

#[derive(Debug)]
pub struct User {
	id: i32,
	username: String,
	password: String
}


pub struct UserDb {
	pool: Pool<PostgresConnectionManager<NoTls>>
}


impl UserDb {
    pub fn builder(client: &mut Client) {
        client.simple_query(TABLE_MODEL).expect("Cannot create client table.");
    }

	pub fn new(pool: Pool<PostgresConnectionManager<NoTls>>) -> Self {
		UserDb { pool }
	}

	pub fn get_all(&self) -> Result<String, Error> {
		let mut res = Vec::new();
		for row in self.pool.get()?.query("SELECT id username password FROM users", &[])? {
			res.push(User {
				id: row.get(0),
				username: row.get(1),
				password: row.get(2)
			})
		}
		Ok(format!("{:?}", res))
	}

    pub fn insert(&self, username: &str, password: &str) -> Result<u64, Error> {
		self.pool.get()?.execute(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            &[&username, &password]
        ).map_err(|e| e.into())
    }
}