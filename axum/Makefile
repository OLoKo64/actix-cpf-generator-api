NAME=cpf-generator-api

debug:
	cargo run

build:
	cargo build --release

test:
	cargo test

docker-build:
	docker build -t $(NAME) .

docker-compose:
	docker-compose up -d

docker-run:
	docker run -d -p 8080:8080 $(NAME)
