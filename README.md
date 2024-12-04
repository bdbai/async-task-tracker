# async-task-tracker

[![crates.io](https://img.shields.io/crates/v/async-task-tracker.svg)](https://crates.io/crates/async-task-tracker)
[![Released API docs](https://docs.rs/async-task-tracker/badge.svg)](https://docs.rs/async-task-tracker)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![CI](https://github.com/bdbai/async-task-tracker/actions/workflows/run-tests.yaml/badge.svg)](https://github.com/bdbai/async-task-tracker/actions/workflows/run-tests.yaml)

A task tracker used for waiting until tasks exit.

This is a modified version of [`TaskTracker`](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html) from the [`tokio-util`](https://docs.rs/tokio-util/latest/tokio_util/index.html) crate without tokio-specific features.

## License

Licensed under [MIT license](LICENSE).

Original work by Tokio Contributors, licensed under [MIT license](https://github.com/tokio-rs/tokio/blob/master/tokio-util/LICENSE).
