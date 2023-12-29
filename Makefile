clean:
	rm -rf /usr/local/bin/ruin
	pkill ruin
	pkill swww

install:
	cargo build --release
	cp target/release/ruin /usr/local/bin
	mkdir -p ~/.ruin

uninstall:
	rm -rf /usr/local/bin/ruin
	rm -rf ~/.ruin
