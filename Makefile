.PHONY: build_and_compare
build_and_compare: build
	@$(MAKE) -s compare

.PHONY: all
all: clean
	@$(MAKE) -s build_and_compare

.PHONY: build
build:
	cargo build
	cargo build --release

.PHONY: compare
compare:
	@find target -mindepth 2 -maxdepth 2 -executable -type f -exec ls -sh '{}' \; | sort -k 1 -h | sed 's/\// /g' | awk '{print $$1, $$4, $$3}' | column -t

.PHONY: clean
clean:
	cargo clean
