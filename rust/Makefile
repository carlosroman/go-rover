CARGO_BIN			?= cargo
DOCKER_BIN			?= docker

.PHONY : fmt
fmt :
	@(rustfmt src/*)

.PHONY : test
test :
	@($(CARGO_BIN) test)

.PHONY : start
start :
	@($(CARGO_BIN) run)

.PHONY : quick-start
quick-start :
	@printf '\n================================================================\n'
	@printf 'Target: quick-start'
	@printf '\n================================================================\n'
	$(DOCKER_BIN) run --rm -it -v ${CURDIR}:/app -w /app rust:1.50.0 make start


target/release/rover :
	$(CARGO_BIN) build --release

build: target/release/rover

.PHONY : lint
lint :
	@($(CARGO_BIN) clippy)
