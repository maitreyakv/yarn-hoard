CARGO_CHECK_CMD = check --all-targets --workspace
CARGO_TEST_CMD = test --workspace --all-targets
CARGO_CLIPPY_CMD = clippy --all-targets --workspace -- -D warnings 
CARGO_FMT_CMD = fmt --check --all

all:
	COMPOSE_BAKE=true docker compose up --watch --build

clean:
	docker compose down
	cargo clean
	trunk clean --config=frontend/Trunk.toml

lint:
	cargo $(CARGO_FMT_CMD)

check:
	cargo $(CARGO_CHECK_CMD)
	cargo $(CARGO_CLIPPY_CMD) 

test:
	cargo $(CARGO_TEST_CMD)

watch:
	cargo watch --no-restart --clear \
		-x "$(CARGO_FMT_CMD)" \
		-x "$(CARGO_CLIPPY_CMD)" \
		-x "$(CARGO_CHECK_CMD)" \
		-x "$(CARGO_TEST_CMD)"

add-migration:
	sea-orm-cli migrate generate "$(NAME)"

build-entities:
	sea-orm-cli generate entity \
		-u postgres://yarn_hoard:password@localhost:5432/yarn_hoard \
		-o entity/src/entities

psql:
	PGPASSWORD=password psql -h 127.0.0.1 -U yarn_hoard
