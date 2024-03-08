REDIRECT_URL := https://mfinelli.github.io/advent-of-code/aoc/index.html

all: target/doc/index.html target/release/aoc

clean:
	rm -rf target

target/doc/index.html: index.html
	cargo doc --locked --no-deps --release
	sed "s|#REDIRECT_URL#|$(REDIRECT_URL)|" $< > $@

target/release/aoc: Cargo.toml Cargo.lock $(wildcard src/*.rs)
	cargo build --locked --release

.PHONY: all clean
