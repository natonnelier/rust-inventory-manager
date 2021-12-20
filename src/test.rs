use rocket::fairing::AdHoc;
use rocket::local::blocking::Client;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Item {
    name: String,
    buy_price_cents: String,
}

fn test(base: &str, stage: AdHoc) {
    // Number of items we're going to create/read/delete.
    const N: usize = 20;

    // NOTE: If we had more than one test running concurently that dispatches
    // DB-accessing requests, we'd need transactions or to serialize all tests.
    let client = Client::tracked(rocket::build().attach(stage)).unwrap();

    // Clear everything from the database.
    assert_eq!(client.delete(base).dispatch().status(), Status::Ok);
    assert_eq!(client.get(base).dispatch().into_json::<Vec<i64>>(), Some(vec![]));

    // Add some random items, ensure they're listable and readable.
    for i in 1..=N{
        let name = format!("Item - {}", i);
        let mut rng = rand::thread_rng();
        let price: i32 = rng.gen();
        let item = Item { name: name.clone(), buy_price_cents: price };

        // Create a new item.
        let response = client.post(base).json(&item).dispatch().into_json::<Item>();
        assert_eq!(response.unwrap(), item);

        // Ensure the index shows one more item.
        let list = client.get(base).dispatch().into_json::<Vec<i64>>().unwrap();
        assert_eq!(list.len(), i);

        // The last in the index is the new one; ensure contents match.
        let last = list.last().unwrap();
        let response = client.get(format!("{}/{}", base, last)).dispatch();
        assert_eq!(response.into_json::<Item>().unwrap(), item);
    }

    // Now delete all of the items.
    for _ in 1..=N {
        // Get a valid ID from the index.
        let list = client.get(base).dispatch().into_json::<Vec<i64>>().unwrap();
        let id = list.get(0).expect("have item");

        // Delete that post.
        let response = client.delete(format!("{}/{}", base, id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    // Ensure they're all gone.
    let list = client.get(base).dispatch().into_json::<Vec<i64>>().unwrap();
    assert!(list.is_empty());

    // Trying to delete should now 404.
    let response = client.delete(format!("{}/{}", base, 1)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_diesel() {
    test("/diesel", crate::diesel_sqlite::stage())
}