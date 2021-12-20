use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};

use rocket_sync_db_pools::diesel;

use self::diesel::prelude::*;

#[database("diesel")]
struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="users"]
struct User {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    name: String,
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

#[table_name="items"]
struct Item {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    name: String,
    user_id: Option<i32>,
    sell_price_cents: Option<i32>,
    buy_price_cents: Option<i32>,
    buy_date: Date,
    sell_date: Date,
}

table! {
    items (id) {
        id -> Nullable<Integer>,
        name -> Text,
        user_id: Nullable<Integer>,
        sell_price_cents: Nullable<Integer>,
        buy_price_cents: Nullable<Integer>,
        buy_date: Date,
        sell_date: Date,
    }
}

#[post("/user", data = "<user>")]
async fn create(db: Db, user: Json<User>) -> Result<Created<Json<User>>> {
    let user_value = user.clone();
    db.run(move |conn| {
        diesel::insert_into(users::table)
            .values(&*user_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/users").body(user))
}

#[get("/users")]
async fn list_users(db: Db) -> Result<Json<Vec<Option<i32>>>> {
    let ids: Vec<Option<i32>> = db.run(move |conn| {
        users::table
            .select(users::id)
            .load(conn)
    }).await?;

    Ok(Json(ids))
}

#[get("users/<id>")]
async fn get_user(db: Db, id: i32) -> Option<Json<User>> {
    db.run(move |conn| {
        users::table
            .filter(users::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()
}

#[delete("users/<id>")]
async fn delete_user(db: Db, id: i32) -> Result<Option<()>> {
    let affected = db.run(move |conn| {
        diesel::delete(users::table)
            .filter(users::id.eq(id))
            .execute(conn)
    }).await?;

    Ok((affected == 1).then(|| ()))
}

#[post("/item", data = "<item>")]
async fn create_user(db: Db, user: Json<Item>) -> Result<Created<Json<Item>>> {
    let item_value = item.clone();
    db.run(move |conn| {
        diesel::insert_into(items::table)
            .values(&*item_value)
            .execute(conn)
    }).await?;

    Ok(Created::new("/items").body(item))
}

#[get("/items")]
async fn list_items(db: Db) -> Result<Json<Vec<Option<i32>>>> {
    let ids: Vec<Option<i32>> = db.run(move |conn| {
        items::table
            .select(items::id)
            .load(conn)
    }).await?;

    Ok(Json(ids))
}

#[get("items/<id>")]
async fn get_item(db: Db, id: i32) -> Option<Json<Item>> {
    db.run(move |conn| {
        items::table
            .filter(items::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()
}

#[delete("items/<id>")]
async fn delete_item(db: Db, id: i32) -> Result<Option<()>> {
    let affected = db.run(move |conn| {
        diesel::delete(items::table)
            .filter(items::id.eq(id))
            .execute(conn)
    }).await?;

    Ok((affected == 1).then(|| ()))
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("db/migrations");

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/diesel", routes![list_users, list_items, get_user, get_item, create_user, create_item, delete_user, delete_item])
    })
}