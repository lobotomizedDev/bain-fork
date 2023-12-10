all:
	cargo build --release

clean:
	su -c 'rm -rf /usr/local/bin/rain'
	pkill rain

install: all
	su -c 'cp target/release/rain /usr/local/bin'
	mkdir -p ~/.rain
	rain &

uninstall:
	cargo clean
	su -c 'rm -rf /usr/local/bin/rain'
	rm -rf ~/.rain
