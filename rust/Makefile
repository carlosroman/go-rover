CARGO_BIN			?= cargo
DOCKER_BIN			?= docker
RUST_VERSION		?= 1.50.0

DOCKER_TTY			?= -t
RUST_DOCKER_CMD		?= $(DOCKER_BIN) run --rm $(DOCKER_TTY) -v $(CURDIR):/app -w /app ci/rust
DOCKER_TARGET_CMD	= $(word 2, $(subst /, ,$(@)))
CI_CMD				?= $(RUST_DOCKER_CMD) make $(DOCKER_TARGET_CMD)

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
quick-start : DOCKER_TTY += -i
quick-start : ci/dep
	@printf '\n================================================================\n'
	@printf 'Target: quick-start'
	@printf '\n================================================================\n'
	@($(RUST_DOCKER_CMD) make start)

target/release/rover :
	$(CARGO_BIN) build --release

build: target/release/rover

.PHONY : lint
lint :
	@($(CARGO_BIN) clippy)

.PHONY : all
all : fmt lint test build

.PHONY : ci/test
ci/test : ci/dep
	@($(CI_CMD))

.PHONY : ci/build
ci/build : ci/dep
	@($(CI_CMD))

.PHONY : ci/lint
ci/lint : ci/dep
	@($(CI_CMD))

.PHONY : ci/all
ci/all : ci/dep
	@($(CI_CMD))

.PHONY : ci/dep
ci/dep :
	@(docker build -t ci/rust -f $(CURDIR)/build/ci/Dockerfile .)
