CARGO_CHECK_CMD = "check --all-targets"
CARGO_TEST_CMD = "test --workspace --all-targets"
CARGO_FMT_CMD = "fmt --check --all"
CARGO_CLIPPY_CMD = "clippy -- -D warnings"

check:
	@cargo $(CARGO_CHECK_CMD)

watch:
	@cargo watch --no-restart --clear \
		-x $(CARGO_FMT_CMD) \
		-x $(CARGO_CLIPPY_CMD) \
		-x $(CARGO_CHECK_CMD) \
		-x $(CARGO_TEST_CMD)

test:
	@cargo $(CARGO_TEST_CMD)

build-entities:
	@sea-orm-cli generate entity \
		-u postgres://yarn_hoard:password@localhost:5432/yarn_hoard \
		-o entity/src/entities

lint:
	@cargo $(CARGO_FMT_CMD)
	@cargo $(CARGO_CLIPPY_CMD) 

psql:
	@PGPASSWORD=password psql -h 127.0.0.1 -U yarn_hoard

clean:
	@cargo clean
