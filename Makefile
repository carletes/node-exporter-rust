.PHONY: all clean

all:
	cross build --target armv7-unknown-linux-gnueabi --release

clean:
	rm -rf target/armv7-unknown-linux-gnueabi
