
TARGET		:= tap
SRC		:= $(shell find ./src/ -name "*.rs")

all: $(TARGET)

$(TARGET): $(SRC)
	cargo build --release

clean:
	rm -rf target/

.PHONY: clean
