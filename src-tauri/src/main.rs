#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod jellyfin;
use std::error::Error;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: &str, password: &str) -> Result<String, String> {
	jellyfin::Jellyfin::authenticate_user(name, password).await
	
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![greet])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
	
	let conn = Connection::open_in_memory();
	match conn {
		Ok(conn) => {
			println!("Connection established");
			match get_database_items(conn){
				Ok(_) => println!("Inserted"),
				Err(_) => println!("Error inserting")
			}
		},
		Err(_) => println!("Error establishing connection")
	}
	;
}


fn get_database_items(conn: Connection) -> Result<(), ()> {

	match conn.execute(
		"CREATE TABLE if not exists person (
			id    INTEGER PRIMARY KEY,
			name  TEXT NOT NULL,
			data  BLOB
		)",
		(), // empty list of parameters.
	) {
		Ok(_) => println!("Created table"),
		Err(_) => println!("Error creating table")
	}

	let mut stmt = conn.prepare("SELECT id, name, data FROM person");

	match stmt {
		Ok(mut stmt) => {
			match stmt.query_map([], |row| {
				Ok(Person {
					id: row.get(0)?,
					name: row.get(1)?,
					data: row.get(2)?,
				})
			}) {
				Ok(person_iter) => {
					for person in person_iter {
						println!("Found person {:?}", person.unwrap());
					}
					Ok(())
				},
				Err(_) => {
					println!("No person found");
					Err(())
				}
			}
		},
		Err(_) => {
			println!("Error preparing statement");
			Err(())
		}
	}

}

fn insert(conn: Connection) -> Result<(), ()> {
	let me = Person {
		id: 0,
		name: "Steven".to_string(),
		data: None,
	};
	match conn.execute(
		"INSERT INTO person (name, data) VALUES (?1, ?2)",
		(&me.name, &me.data),
	) {
		Ok(_) => Ok(()),
		Err(_) => Err(())
	}
}