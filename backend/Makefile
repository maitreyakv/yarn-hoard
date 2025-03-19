check:
	@cargo check

lint:
	@cargo fmt --check
	@cargo clippy -- -D warnings

psql:
	@PGPASSWORD=password psql -h 127.0.0.1 -U yarn_hoard

clean:
	@cargo clean
