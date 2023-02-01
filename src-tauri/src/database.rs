use rusqlite::Connection;

use crate::jellyfin::ClientData;

fn connect_to_database() -> Connection {
	println!("Connecting to database");
	Connection::open("rustyfin.db").unwrap()
}

pub fn save_user(client_data: &ClientData) -> Result<String, String> {

	if let Some(access_token) = &client_data.access_token {
		println!("Access token: {}", access_token);

		let conn = connect_to_database();

		conn.execute(
			"CREATE TABLE IF NOT EXISTS user (
				id    INTEGER PRIMARY KEY,
				accesstoken  TEXT NOT NULL
			)",
			(), // empty list of parameters.
		).expect("Failed to create table");
		match conn.execute(
			"INSERT INTO user (accesstoken) VALUES (?)",
			[&access_token.to_string()],
		) {
			Ok(_) => Ok("Inserted".to_string()),
			Err(err) => Err(err.to_string())
		}

	} else {
		return Err("No access token".to_string());
	}
}

#[derive(Debug)]
pub struct SavedUser {
	id: i32,
	accesstoken: String,
}

pub fn get_user() -> Result<SavedUser, String> {
	let conn = connect_to_database();

	let mut stmt = conn.prepare("SELECT id, accesstoken FROM user").expect("Error preparing statement");
	
	let mut user_iter = stmt.query_map([], |row| {
		Ok(SavedUser {
			id: row.get(0)?,
			accesstoken: row.get(1)?,
		})
	}).expect("Error mapping query to User struct");

	let user: SavedUser = user_iter.next().unwrap().unwrap();

	// Get the first user
	Ok(user)
}