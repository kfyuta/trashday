#[derive(Queryable)]
pub struct TrashDay {
    pub trash_type: String,
    pub nth: i32,
    pub day: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}