#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod jellyfin;
mod database;
use std::{error::Error, io};
use database::{SavedUser, get_user};
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

#[tauri::command]
async fn get_saved_user() -> String {
	let user = get_user().unwrap();

	format!("{:?}", user)
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![greet, get_saved_user])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");	
}