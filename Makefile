

start_db:
	@docker run --name quests-tracker-db -p 5432:5432 -e POSTGRES_PASSWORD=123456 -d postgres:17

migrate_db:
	@cd src/infrastructure/postgres && diesel migration generate init_database