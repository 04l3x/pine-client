use crate::graphql::Executor;
use error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Visibility {
	Public,
	Private,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
	pub id: Uuid,
	pub owner_id: Uuid,
	pub name: String,
	pub description: String,
	pub visibility: Visibility,
	pub owner_name: String,
	pub owner_avatar: String,
}

impl Record {
	pub async fn get_public_record(page: i32) -> Result<Data> {
		let vars = Vars {
			page,
			filter: None,
			query: None,
		};

		Executor::default()
			.execute_with_vars::<Data, Vars>("public_record_paginated", vars)
			.await
	}

	pub async fn search(query: String, page: i32) -> Result<Data> {
		let vars = Vars {
			page,
			filter: None,
			query: Some(query),
		};

		Executor::default()
			.execute_with_vars::<Data, Vars>("public_record_with_vars", vars)
			.await
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	public_record: Records,
}

impl Data {
	pub fn content(self) -> Vec<Record> {
		self.public_record.results
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Info {
	_count: i32,
	_pages: i32,
	_prev: Option<i32>,
	_next: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Records {
	_info: Info,
	results: Vec<Record>,
}

#[derive(Serialize)]
struct Vars {
	page: i32,
	filter: Option<Filter>,
	query: Option<String>,
}

#[derive(Serialize)]
struct Filter {
	id: Option<Uuid>,
	name: Option<String>,
	description: Option<String>,
	visibility: Option<Visibility>,
}
