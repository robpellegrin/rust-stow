TARGET = rstow
INSTALL_DIR= $(HOME)/.cargo/bin/$(TARGET)

build:
	cargo build --release

install:
	mkdir -p ~/.local/bin/
	ln -sf $(PWD)/target/release/$(TARGET) $(INSTALL_DIR) 

clean:
	cargo clean
