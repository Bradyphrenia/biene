use serde:{Deserialze, Serialize};

#[derive(Debug, Serialize, Deserialze)]
pub struct Test {
    pub id: i32,
    pub name: &str,
    pub value: i32,
    pub description: &str
}
