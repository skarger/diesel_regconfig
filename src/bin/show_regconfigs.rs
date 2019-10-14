extern crate diesel_regconfig;
extern crate diesel;

use self::diesel_regconfig::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use crate::schema::example_rows::dsl::*;

    let connection = establish_connection();
    let results = example_rows
        .limit(5)
        .load::<ExampleRow>(&connection)
        .expect("Error loading example_rows");

    println!("Displaying {} regconfig values:", results.len());
    for example_row in results {
        println!("{}: {}", example_row.id, example_row.ts_config_name);
    }
}