use crate::schema::posts;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug, Identifiable, PartialEq, Eq, AsChangeset)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
