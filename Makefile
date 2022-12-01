REDIRECT_URL := https://mfinelli.github.io/advent-of-code/aoc/index.html

all: target/doc/index.html

target/doc/index.html: index.html
	sed "s|#REDIRECT_URL#|$(REDIRECT_URL)|" $< > $@

.PHONY: all
