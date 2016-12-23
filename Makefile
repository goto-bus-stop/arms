all: rocks crates

clean:
	$(MAKE) -C rocks/arms clean
	$(MAKE) -C crates/scx clean
	$(MAKE) -C crates/arms clean

rocks:
	$(MAKE) -C rocks/arms

crates: rocks
	$(MAKE) -C crates/scx
	$(MAKE) -C crates/arms

.PHONY: all rocks crates clean
