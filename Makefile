LCOV_FILE ?= coverage.lcov

test:
	cargo test -p translatable -- --nocapture --color=always --test-threads=1

cov:
ifdef export-lcov
	@echo "Generating LCOV report..."
	@coverage=$$(cargo llvm-cov -- --nocapture --test-threads=1 --color=never | grep '^TOTAL' | awk '{print $$10}'); \
	cargo llvm-cov --lcov -- --nocapture --test-threads=1 --color=always > $(LCOV_FILE); \
	echo "LCOV report saved to $(LCOV_FILE)"; \
	echo "Total Coverage: $$coverage%"
else
	@coverage=$$(cargo llvm-cov -- --nocapture --test-threads=1 --color=never | grep '^TOTAL' | awk '{print $$10}'); \
	echo "Total Coverage: $$coverage%"
endif
