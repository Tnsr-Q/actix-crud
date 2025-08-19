.PHONY:run
.PHONY:build

run:
		RUST_LOG=info cargo watch -x run

build:
			cargo build
