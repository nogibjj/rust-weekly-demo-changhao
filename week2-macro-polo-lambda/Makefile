format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

run:
	cargo run

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
		--data-ascii '{"name": "Marco"}' \
		--output-format json \
		macro-polo-lambda

all: format lint run