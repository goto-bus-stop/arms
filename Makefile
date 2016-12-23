all: rocks crates

rocks:
	$(MAKE) -C rocks/arms compile

crates: rocks
	$(MAKE) -C crates/scx
	$(MAKE) -C crates/arms

.PHONY: rocks crates
