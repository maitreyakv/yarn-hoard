check:
	@cargo check --all-targets

watch:
	@cargo watch --no-restart --clear -x "check --all-targets" -x "test --workspace --all-targets"

test:
	@cargo test --workspace --all-targets

build-entities:
	@sea-orm-cli generate entity \
		-u postgres://yarn_hoard:password@localhost:5432/yarn_hoard \
		-o entity/src/entities

lint:
	@cargo fmt --check
	@cargo clippy -- -D warnings

psql:
	@PGPASSWORD=password psql -h 127.0.0.1 -U yarn_hoard

clean:
	@cargo clean
