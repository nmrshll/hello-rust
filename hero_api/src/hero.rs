#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: Option<String>,
    pub hometown: Option<String>,
    pub age: Option<i32>,
}
