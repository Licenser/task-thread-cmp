bench:
	cargo build --all --release
	strip target/release/tasks
	strip target/release/threads
	@echo task
	taskset -c 0,1,2,24,25,26 time target/release/tasks
	@echo thread
	taskset -c 0,1,2,24,25,26 time target/release/threads

