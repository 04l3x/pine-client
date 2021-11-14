use lightql::{Client, GraphQLError};
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
	pub async fn get_public_record(page: i32) -> Result<Data, GraphQLError> {
		let vars = Vars { page, filter: None, query: None };
		Client::new(crate::ENDPOINT)
			.query_with_vars::<Data, Vars>(
				r#"
				query public_record_paginated($page: Int) {
					publicRecord(page: $page) {
						info {
							count
							pages
							prev
							next
						}
						results {
							id
							name
							ownerId
							description
							visibility,
							ownerName,
							ownerAvatar
						}
					}
				}
			"#,
				vars,
			)
			.await
	}

	pub async fn search(query: String, page: i32) -> Result<Data, GraphQLError> {
		let vars = Vars {
			page,
			filter: None,
			query: Some(query),
		};

		Client::new(crate::ENDPOINT)
			.query_with_vars::<Data, Vars>(
				r#"
				query public_record_paginated_with_filter($page: Int, $query: String) {
					publicRecord(page: $page, query: $query) {
						info {
							count
							pages
							prev
							next
						}
						results {
							id
							name
							ownerId
							description
							visibility
							ownerName
							ownerAvatar
						}
					}
				}
			"#,
				vars,
			)
			.await
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	public_record: Records,
}

impl Data {
	pub fn content(self) -> Option<Vec<Record>> {
		self.public_record.results
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Info {
	count: i32,
	pages: i32,
	prev: Option<i32>,
	next: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Records {
	info: Info,
	results: Option<Vec<Record>>,
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
