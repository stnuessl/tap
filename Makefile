
TARGET		:= tap
SRC		:= $(shell find ./src/ -name "*.rs")
DEST_DIR	:= /usr/local/bin

all: $(TARGET)

$(TARGET): $(SRC)
	cargo build --release

install: $(TARGET)
	cp ./target/release/$(TARGET) $(DEST_DIR) 

uninstall:
	rm -f $(DEST_DIR)/$(TARGET)
clean:
	rm -rf target/

.PHONY: clean uninstall
