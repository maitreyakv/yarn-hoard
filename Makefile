all:
	@COMPOSE_BAKE=true docker compose up --watch --build

check:
	@$(MAKE) -C backend $@

psql:
	@$(MAKE) -C backend $@

clean:
	@docker compose down
	@$(MAKE) -C backend $@
