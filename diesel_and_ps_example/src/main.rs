extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::Post;

fn main() {
    let mut conn = establish_connection();
    let new_post = create_post(&mut conn, &1, "Hello", "World");
    println!("{:?}", new_post);
    println!("{}", get_posts(&mut conn));
}

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(
    conn: &mut PgConnection,
    id_loc: &i32,
    title: &str,
    body: &str,
) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts;

    let new_seaker = Post {
        id: *id_loc,
        title: String::from(title),
        body: String::from(body),
        published: true,
    };

    diesel::insert_into(posts::table)
        .values(&new_seaker)
        .get_result(conn)
}

pub fn get_posts(conn: &mut PgConnection) -> String {
    use schema::posts::dsl::*;

    let res: Vec<Post>;

    match posts.load::<Post>(conn) {
        Ok(good) => res = good,
        Err(bad) => return format!("{}", bad),
    }
    let mut return_val = String::new();

    for item in res {
        return_val += &format!(
            "**Team ID**: {}\n**Team Name**: {}\n**Leader**: {}\n**Members**: {}\n\n",
            item.id, item.title, item.body, item.published
        )
        .to_owned();
    }

    if return_val == "" {
        return_val = String::from("No teams to display");
    }

    format!("{}\n", return_val)
}
