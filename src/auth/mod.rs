use crate::graphql::Executor;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, lightql::GraphQLError>;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub token: String,
}

impl Session {
    fn save(&self) {
        LocalStorage::set("session", self).unwrap();
    }

    fn _clean() {
        LocalStorage::delete("session")
    }

    pub fn _get_current() -> std::result::Result<Session, ()> {
        match LocalStorage::get("session") {
            Ok(token) => Ok(token),
            Err(_) => Err(()),
        }
    }

    /*pub async fn is_logged() -> bool {
        match Executor::default_headers()
            .execute::<Data>("is_logged")
            .await
        {
            Ok(result) => result.is_logged.expect("no logged information"),
            Err(_) => false,
        }
    }*/
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
    //is_logged: Option<bool>,
}
