use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    company: String,
}

fn createUser(name: String, company: String) -> User {
    User { name, company }
}
