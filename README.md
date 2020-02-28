# task-thread-cmp

```shell
cargo build --all --release

taskset -c 0,1,2 time target/release/tasks
taskset -c 0,1,2 time target/release/threads
```
