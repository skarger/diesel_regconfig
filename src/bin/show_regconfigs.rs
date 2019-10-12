extern crate diesel_regconfig;
extern crate diesel;

use self::diesel_regconfig::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use diesel_regconfig::schema::regconfigs::dsl::*;

    let connection = establish_connection();
    let results = regconfigs
        .limit(5)
        .load::<RegConfig>(&connection)
        .expect("Error loading regconfigs");

    println!("Displaying {} regconfigs", results.len());
    for regconfig in results {
        println!("{}: {}", regconfig.id, "<column value>");
    }
}