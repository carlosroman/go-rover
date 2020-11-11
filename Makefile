# Default GO_BIN to Go binary in PATH
GO_BIN				?= go

TEST_PATTERN ?=.
TEST_OPTIONS ?=
SOURCE_FILES ?= ./...

TEST_FLAGS += -failfast
TEST_FLAGS += -race

GO_TEST ?= test $(TEST_OPTIONS) $(TEST_FLAGS) $(SOURCE_FILES) -run $(TEST_PATTERN) -timeout=10m

.PHONY: go-get
go-get:
	@printf '\n================================================================\n'
	@printf 'Target: go-get'
	@printf '\n================================================================\n'
	$(GO_BIN) mod vendor
	@echo '[go-get] Done.'

.PHONY: test-coverage
test-coverage: TEST_FLAGS += -covermode=atomic -coverprofile=coverage.out
test-coverage: go-get
	@printf '\n================================================================\n'
	@printf 'Target: test-coverage'
	@printf '\n================================================================\n'
	@echo '[test] Testing packages: $(SOURCE_FILES)'
	$(GO_BIN) $(GO_TEST)
