use std::collections::HashMap;

pub fn load<'q>(key: &'q str) -> Option<&'q str> {
	Some(*queries::<'q>().get(key).expect("error loadind query"))
}

///queries.insert(
///	"$$query_name$$",
///	"$$query_raw$$",
///);
fn queries<'q>() -> HashMap<&'q str, &'q str> {
	let mut queries: HashMap<&str, &str> = HashMap::new();

	queries.insert(
		"login",
		"mutation login($creds: Credentials) {
			signIn(credentials: $creds){
				token
			}
		}",
	);

	queries.insert(
		"register",
		"mutation register($form: SignUpForm) {
			signUp(form: $form){
				token
			}
		}",
	);

	queries.insert(
		"is_logged",
		"query is_logged {
			isLogged
		}",
	);

	queries.insert(
		"public_record_paginated",
		"query public_record_paginated($page: Int) {
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
		}",
	);

	queries.insert(
		"public_record_with_vars",
		"query public_record_paginated_with_filter($page: Int, $query: String) {
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
		}",
	);

	queries
}
