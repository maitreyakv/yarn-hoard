all:
	@COMPOSE_BAKE=true docker compose up --watch --build

check:
	@cd backend && cargo check

clean:
	@docker compose down
	@cd backend && cargo clean
