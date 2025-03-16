all:
	@COMPOSE_BAKE=true docker compose up --watch

check:
	@cd backend && cargo check

clean:
	@cd backend && cargo clean
