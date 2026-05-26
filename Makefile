COVERAGE_PACKAGES := -p web_server_binding

node_modules: package-lock.json
	npm ci
	touch node_modules

package-lock.json: package.json
	npm install --package-lock-only

.PHONY: clean
clean:
	rm -rf node_modules target

.PHONY: clippy
clippy:
	cargo clippy --all-targets

.PHONY: coverage
coverage: node_modules
	cargo llvm-cov clean --workspace
	cargo llvm-cov $(COVERAGE_PACKAGES) --no-report
	cargo llvm-cov report --json --output-path target/llvm-cov.json
	cargo llvm-cov report --lcov --output-path target/lcov.info
	cargo llvm-cov report
	npx rust-coverage-check target/llvm-cov.json \
		--workspace-root $(CURDIR) \
		--gated web_server_binding=100

.PHONY: coverage-clean
coverage-clean:
	cargo llvm-cov clean --workspace
	rm -rf target/llvm-cov-target
	rm -f target/llvm-cov.json target/lcov.info

.PHONY: coverage-report
coverage-report: node_modules
	cargo llvm-cov $(COVERAGE_PACKAGES) --html

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test
