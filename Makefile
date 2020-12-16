# Notes:
# - Adopted the help target from https://szymonkrajewski.pl/use-make-as-task-runner/
# - Adopted guard trick from https://www.robg3d.com/2020/05/beautiful-makefiles-with-wildcards/

guard-cmd-%:
	@if ! which '${*}' &> /dev/null; then echo 'ERROR: command $* not in PATH' && exit 1; fi
.PHONY: guard-cmd-%

guard-%:
	@if [ -z '${${*}}' ]; then echo 'ERROR: variable $* not set' && exit 1; fi
.PHONY: guard-%

## Format rust
fmt:
	@cargo fmt
.PHONY: rust-fmt

## Lint
lint:
	@cargo clippy -- -D warnings
.PHONY: lint

## Audit
audit:
	@cargo audit
.PHONY: audit

## Run cargo tarpaulin
coverage:
	@cargo tarpaulin --ignore-tests
.PHONE: coverage


GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

TARGET_MAX_CHAR_NUM=15

.DEFAULT_GOAL := help

## Show this help message
help:
# Parse all targets and their help texts in this Makefile.  Target
# names are parsed as identifiers followed ":".  The help texts are in
# the line before its target, starting with "## ".
	@echo ''
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-_0-9]+:/ { \
		helpMessage = match(lastLine, /^## (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")); \
			sub(/:/, "", helpCommand); \
			helpMessage = substr(lastLine, RSTART + 3, RLENGTH); \
			printf "  ${YELLOW}%-$(TARGET_MAX_CHAR_NUM)s${RESET} ${GREEN}%s${RESET}\n", helpCommand, helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)
