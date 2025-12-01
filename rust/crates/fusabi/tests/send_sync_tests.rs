// Tests to verify that Engine and related types implement Send + Sync
// This addresses issue #146: Make Engine Send + Sync for async/await integration

use fusabi::Engine;

#[test]
fn test_engine_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Engine>();
}

#[test]
fn test_engine_is_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Engine>();
}

#[test]
fn test_engine_works_with_tokio_spawn() {
    // This test verifies that Engine can be used with tokio::spawn
    // which requires Send + 'static

    // Create a simple async runtime test without actually using Tokio
    // to avoid adding a dependency just for tests
    fn requires_send_sync<T: Send + Sync>(_: T) {}

    let engine = Engine::new();
    requires_send_sync(engine);
}

#[test]
fn test_engine_can_be_shared_across_threads() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let engine = Arc::new(Mutex::new(Engine::new()));
    let engine_clone = engine.clone();

    let handle = thread::spawn(move || {
        // Engine should be usable in another thread
        let eng = engine_clone.lock().unwrap();
        // Check that stdlib functions are registered
        assert!(eng.host_function_names().len() > 0);
    });

    handle.join().unwrap();
}
