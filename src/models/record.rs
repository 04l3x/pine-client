use lightql::{Client, GraphQLError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Visibility {
	Public,
	Private,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
	pub id: Uuid,
	pub owner_id: Uuid,
	pub name: String,
	pub description: String,
	pub visibility: Visibility,
}

impl Record {
	pub async fn get_public_record() -> Result<Data, GraphQLError> {
		Client::new(crate::ENDPOINT)
			.query::<Data>(
				r#"
				query all_public_repositories {
					publicRecord(page: 1) {
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
						}
					}
				}
			"#,
			)
			.await
	}

	pub async fn search(query: String, page: i32) -> Result<Data, GraphQLError> {
		let vars = Vars {
			page,
			filter: Filter {
				id: None,
				name: Some(query),
				description: None,
				visibility: None,
			},
		};

		Client::new(crate::ENDPOINT)
			.query_with_vars::<Data, Vars>(
				r#"
				query all_public_repositories {
					publicRecord(page: 1) {
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
						}
					}
				}
			"#,
			vars).await
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
	results: Option<Vec<Record>>
}

#[allow(dead_code)]
#[derive(Serialize)]
struct Vars {
	page: i32,
	filter: Filter,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct Filter {
  id: Option<Uuid>,
  name: Option<String>,
  description: Option<String>,
  visibility: Option<Visibility>,
}



