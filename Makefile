APP_NAME = math-engine
CARGO_FLAGS = --release --no-default-features
SERVER_TARGET = aarch64-unknown-linux-gnu

.PHONY: clean all
all: win linux
	mkdir bin
	find target -type f -executable -name "$(APP_NAME)*" -exec cp {} bin/ \; -print

clean:
	cargo clean
	rm -vrf bin

win:
	cargo build $(CARGO_FLAGS) --target x86_64-pc-windows-gnu

linux:
	cargo build $(CARGO_FLAGS) --target x86_64-unknown-linux-gnu