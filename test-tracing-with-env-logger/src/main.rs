use bytes::Bytes;
use slatedb::config::DbOptions;
use slatedb::db::Db;
use slatedb::fail_parallel::FailPointRegistry;
use slatedb::object_store::{memory::InMemory, ObjectStore};
use std::sync::Arc;
use log::{debug, error, log_enabled, warn, info, Level, trace};


#[tokio::main]
async fn main() {

    // enable env logger
    env_logger::init();

    // test trace log
    if log_enabled!(Level::Trace) {
        let x = 3 * 4; // expensive computation
        trace!("the answer was: {}", x);
    }

    // test logs levels
    debug!("this is a debug {}", "message");
    error!("this is printed by default");
    info!("this is printed by default");
    warn!("this is printed by default");
    log_enabled!(Level::Info);

    // Setup
    let object_store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
    let options = DbOptions::default();
    let fp_registry = Arc::new(FailPointRegistry::new());

    // Open the slatedbdatabase - this will trigger the logging
    let db = Db::open_with_fp_registry("/tmp/test_kv_store2", options, object_store, fp_registry)
        .await
        .expect("Failed to open database");

    // Put
    let key = b"test_key";
    let value = b"test_value";
    db.put(key, value).await;

    // Get
    assert_eq!(db.get(key).await.unwrap(), Some(Bytes::from_static(value)));

    // Delete
    db.delete(key).await;
    assert!(db.get(key).await.unwrap().is_none());

    // Close
    db.close().await.unwrap();
}
