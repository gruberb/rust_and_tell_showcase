// use chrono::{NaiveDate};
use super::schema::talks;

#[derive(Queryable)]
pub struct Talk {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub published: bool,
    // pub video_link: String,
    // pub slides_link: String,
    // pub created_at: NaiveDate,
    // pub updated_at: NaiveDate,
}

#[derive(Insertable)]
#[table_name="talks"]
pub struct NewTalk<'a> {
    pub title: &'a str,
    pub description: &'a str,
}