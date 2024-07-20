use custom_macro::MyDebug;

#[derive(MyDebug)]
pub struct User {
    pub name: String,
    pub id: i64,
}
