
build:
	cargo install --path .

shell:
	docker exec -it bjira_web_1 /bin/bash

make dev-run:
	cargo watch -x 'run --bin bjira'

restart:
	docker-compose stop
	docker-compose start

make dc-build:
	docker-compose rm
	docker-compose up -d
