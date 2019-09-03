use postgres::{ Client, error::Error };

const TABLE_MODEL: &str = "CREATE TABLE users (
        id         SERIAL PRIMARY KEY,
        username   TEXT NOT NULL,
        password   TEXT NOT NULL
    )";

pub struct User<'a> { client: &'a mut Client }

impl<'a> User<'a> {
    pub fn builder(client: &'a mut Client) -> User {
        client.simple_query(TABLE_MODEL).expect("Cannot create client table.");
        User { client }
    }

    pub fn insert(&mut self, username: &str, password: &str) -> Result<u64, Error> {
        self.client.execute(
            "INSERT INTO users (username, password) VALUES ($1, $2)",
            &[&username, &password]
        )
    }
}