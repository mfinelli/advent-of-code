set dotenv-load := false
set ignore-comments := true

year := datetime("%Y")
day := datetime("%d")

[private]
default:
    @just --list

[private]
build:
    @cargo build --release --quiet

# Runs today's puzzle
run-today: build
    ./target/release/aoc {{ year }} {{ day }} \
        ./input/{{ year }}/day{{ day }}.txt

# Runs the puzzle for the given year/day
run YEAR DAY: build
    @./target/release/aoc {{ YEAR }} {{ DAY }} \
        ./input/{{ YEAR }}/day`printf '%02d' {{ DAY }}`.txt

# Runs the tests for today's puzzle
test-today:
    @echo "{{ BOLD + WHITE }}cargo test y`cut -c 3- \
        <<< {{ year }}`d{{ day }}{{ NORMAL }}"
    @cargo test y`cut -c 3- <<< {{ year }}`d{{ day }}

# Runs the tests for the given year/day
test YEAR DAY:
    @echo "{{ BOLD + WHITE }}cargo test y`cut -c 3- \
        <<< {{ YEAR }}`d`printf '%02d' {{ DAY }}`{{ NORMAL }}"
    @cargo test y`cut -c 3- <<< {{ YEAR }}`d`printf '%02d' {{ DAY }}`
