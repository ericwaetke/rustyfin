use reqwest;
use reqwest::header;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() {
	let username = "USERNAME";
	let password = "PASSWORD";


	authenticate_user(&username, &password).await;
}

async fn authenticate_user(username: &str, password: &str) -> Result<(), Box<dyn Error>> {
	println!("Attempting to authenticate user {} with password {}", username, password);
	let jellyfin_server_url: &str  = "https://JELLYFIN";
	let request_url = format!("{}{}", jellyfin_server_url, "/users/AuthenticateByName");


	let mut body = HashMap::new();
	body.insert("Username", username);
	body.insert("Pw", password);

	let mut headers = header::HeaderMap::new();
	headers.insert("accept", header::HeaderValue::from_static("application/json"));
	headers.insert("content-type", header::HeaderValue::from_static("application/json"));
	headers.insert("X-Application", header::HeaderValue::from_static("Rustyfin/0.0.1"));
	headers.insert("Accept-Charset", header::HeaderValue::from_static("UTF-8,*"));
	headers.insert("Accept-encoding", header::HeaderValue::from_static("gzip"));
	headers.insert("User-Agent", header::HeaderValue::from_static("Rustyfin/0.0.1"));
	headers.insert("X-Emby-Authorization", header::HeaderValue::from_static("MediaBrowser Client='Rustyfin', Device='Windows', DeviceId='1', Version='0.0.1'"));

	println!("headers: {:?}", headers);
	println!("body: {:?}", body);

	let client = reqwest::Client::new();
	let res = client
		.post(request_url)
		.json(&body)
		.headers(headers)
		.timeout(Duration::from_secs(3))
		.send()
		.await?
		.text()
		.await?;
	println!("{:}", res);
	Ok(())
}

async fn get_jellyfin_server_info() -> Result<(), Box<dyn Error>> {
	let jellyfin_server_url: &str  = "https://jellyfin.ericwaetke.de";
	let request_url = format!("{}{}", jellyfin_server_url, "/System/Info/Public");

	let client = reqwest::Client::new();
	let doge = client
		.get(request_url)
		.header("Accept", "application/json")
		.timeout(Duration::from_secs(3))
		.send()
		.await?
		.text()
		.await?;
	println!("{:}", doge);
	Ok(())
}