mod queries;
//use crate::utils;
use error::Result;
use lightql::Client;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;

pub struct Executor<'c> {
	client: Client<'c>,
}

impl<'c> Default for Executor<'c> {
	fn default() -> Self {
		Self {
			client: Client::<'c>::new(crate::endpoint()),
		}
	}
}

/// for unlogged queries
///
/// Executor::default()
///
/// for logged queries
///
/// Executor::default_headers()
///
impl<'c> Executor<'c> {
	/*pub fn default_headers() -> Self {
		let token = Session::get_token();

		let mut headers: HashMap<&str, &str> = HashMap::new();
		headers.insert("token", token.as_str());

		Self {
			client: Client::<'c>::new_with_headers(crate::endpoint(), headers),
		}
	}*/

	/*pub async fn execute<'q, T>(self, query_name: &'q str) -> Result<T>
	where
		T: for<'de> Deserialize<'de>,
	{
		match self.client.query::<T>(QueryLoader::load(query_name)).await {
			Ok(result) => Ok(result),
			Err(e) => Err(Box::new(e)),
		}
	}*/

	pub async fn execute_with_vars<'q, T, V>(self, query_name: &'q str, vars: V) -> Result<T>
	where
		T: for<'de> Deserialize<'de>,
		V: Serialize,
	{
		match self
			.client
			.query_with_vars::<T, V>(QueryLoader::load(query_name), vars)
			.await
		{
			Ok(result) => Ok(result),
			Err(e) => Err(Box::new(e)),
		}
	}
}

struct QueryLoader;

impl QueryLoader {
	fn load<'q>(query_name: &'q str) -> &'q str {
		queries::load(query_name).expect("error loading query")
	}
}

//#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Session {
	token: String,
}

/*impl Session {
	fn get_token() -> String {
		match utils::local_storage().restore("session") {
			Ok(session) => match serde_json::from_str::<Session>(&session) {
				Ok(s) => s.token,
				Err(_) => String::from(""),
			},
			Err(_) => String::from(""),
		}
	}
}*/
