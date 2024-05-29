pub mod models;
pub mod schema;

use std::env;
use diesel::PgConnection;
use dotenvy::dotenv;
use models::file::File;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::files::dsl::*;

    let connection = &mut establish_connection();
    let results = files
        .limit(5)
        .select(File::as_select())
        .load(connection)
        .expect("Error loading files");

    println!("Displaying {} files", results.len());
    for file in results {
        println!("{}", file.id);
        println!("{}", file.filepath);
        println!("{}", file.size);
        println!("{}", file.replicas);
        println!("{}", file.updated_at);
    }
}