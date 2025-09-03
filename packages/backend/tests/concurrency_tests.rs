// Concurrency tests for POE2 Overlord backend
// Tests for Arc reference counting and tokio channels

#[test]
fn test_arc_cloning() {
    use std::sync::Arc;

    let data = Arc::new(42);
    let cloned = data.clone();

    assert_eq!(*data, 42);
    assert_eq!(*cloned, 42);
    assert_eq!(Arc::strong_count(&data), 2);
    assert_eq!(Arc::strong_count(&cloned), 2);
}

#[test]
fn test_mpsc_channel() {
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel::<i32>(10);

    // Send some values
    tx.blocking_send(1).unwrap();
    tx.blocking_send(2).unwrap();
    tx.blocking_send(3).unwrap();

    // Close the sender
    drop(tx);

    // Receive values
    assert_eq!(rx.blocking_recv(), Some(1));
    assert_eq!(rx.blocking_recv(), Some(2));
    assert_eq!(rx.blocking_recv(), Some(3));
    assert_eq!(rx.blocking_recv(), None); // Channel closed
}
