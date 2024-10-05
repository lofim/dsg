.PHONY: data
data:
	cargo run > ./tests/init/insert-users.sql

.PHONY: data-release
data-release:
	cargo build --release
	time ./target/release/dsg > ./tests/init/insert-users.sql

.PHONY: clean_db
clean_db:
	docker kill `docker ps -qa` || true
	docker rm `docker ps -qa` || true
	docker volume rm `docker volume ls -q` || true

.PHONY: db_up
db_up:
	docker-compose -f tests/postgres.docker-compose.yml up
