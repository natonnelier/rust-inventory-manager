#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

#[cfg(test)] mod tests;

mod diesel_sqlite;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(diesel_sqlite::stage())
}