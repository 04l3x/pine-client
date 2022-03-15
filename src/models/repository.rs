use crate::graphql::Executor;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Result<T> = std::result::Result<T, lightql::GraphQLError>;

pub struct Repository;

impl Repository {
	pub async fn get_tree(id: Uuid) -> Result<Data> {
		let vars = Vars { id };

		Executor::default()
			.execute_with_vars::<Data, Vars>("repo_tree", vars)
			.await
	}
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
	repo_tree: Tree,
}

impl Data {
	pub fn content(self) -> Tree {
		self.repo_tree
	}
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct Info {
	count: i32,
	pages: i32,
	prev: Option<i32>,
	next: Option<i32>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
	pub objects: Vec<Object>,
}

impl Default for Tree {
	fn default() -> Tree {
		Tree { objects: vec![] }
	}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Object {
	pub path: String,
	pub kind: ObjectKind,
	pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ObjectKind {
	Blob,
	Tree,
	Commit,
	Any,
	Tag,
}

#[derive(Serialize)]
struct Vars {
	id: Uuid,
}
