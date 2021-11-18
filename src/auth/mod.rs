use crate::graphql::Executor;
use error::Result;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
	pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
	username: String,
	password: String,
}

impl Credentials {
	pub fn new(username: String, password: String) -> Self {
		Credentials { username, password }
	}

	pub async fn login(self) -> Result<()> {
		let vars = Vars {
			creds: Some(self),
			form: None,
		};

		match Executor::default()
			.execute_with_vars::<Data, Vars>("login", vars)
			.await
		{
			Ok(data) => {
				data.sign_in.expect("").save();
				Ok(())
			}
			Err(e) => Err(e),
		}
	}
}

#[derive(Debug, Serialize)]
pub struct User {
	email: String,
	username: String,
	password: String,
}

impl User {
	pub fn new(email: String, username: String, password: String) -> Self {
		User {
			email,
			username,
			password,
		}
	}

	pub async fn register(self) -> Result<()> {
		let vars = Vars {
			creds: None,
			form: Some(self),
		};

		match Executor::default()
			.execute_with_vars::<Data, Vars>("register", vars)
			.await
		{
			Ok(data) => {
				data.sign_up.expect("").save();
				Ok(())
			}
			Err(e) => Err(e),
		}
	}
}

impl Session {
	fn save(&self) {
		StorageService::new(Area::Local)
			.expect("not storage available")
			.store("session", Json(self));
	}
}

fn is_logged() -> bool {
	false
}

#[derive(Debug, Serialize)]
struct Vars {
	creds: Option<Credentials>,
	form: Option<User>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
	sign_in: Option<Session>,
	sign_up: Option<Session>,
}
