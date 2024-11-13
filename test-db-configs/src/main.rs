use bytes::Bytes;
use slatedb::db::Db;
use slatedb::config::DbOptions;
use slatedb::fail_parallel::FailPointRegistry;
use slatedb::object_store::{ObjectStore, memory::InMemory};
use std::sync::Arc;

#[tokio::main]
async fn main() {

    // Initialize tracing subscriber to see the logs
    tracing_subscriber::fmt::init();

    // Setup
    let object_store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
    let options = DbOptions::default();
    let fp_registry = Arc::new(FailPointRegistry::new());

     // Open the database - this will trigger the logging
    let db = Db::open_with_fp_registry(
        "/tmp/test_kv_store1",
        options,
        object_store,
        fp_registry,
    ).await.expect("Failed to open database");

    //let kv_store = Db::open_with_opts(
    //    "/tmp/test_kv_store",
    //    options,
    //    object_store,
    //  )
    //.await
    //.unwrap();

    // Put
    let key = b"test_key";
    let value = b"test_value";
    db.put(key, value).await;

    // Get
    assert_eq!(
        db.get(key).await.unwrap(),
        Some(Bytes::from_static(value))
    );

    // Delete
    db.delete(key).await;
    assert!(db.get(key).await.unwrap().is_none());

    // Close
    db.close().await.unwrap();
}
