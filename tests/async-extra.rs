//! Extra tests to make sure the TaskTracker works with event-listener.

use async_task_tracker::TaskTracker;
use tokio_test::{assert_pending, task};

#[test]
fn multiple_listener() {
    let tracker = TaskTracker::new();
    tracker.close();
    let token = tracker.token();

    let mut wait1 = task::spawn(tracker.wait());
    let mut wait2 = task::spawn(tracker.wait());

    assert_pending!(wait1.poll());
    assert_pending!(wait2.poll());
    drop(token);
    assert!(wait1.is_woken());
    assert!(wait2.is_woken());
}
