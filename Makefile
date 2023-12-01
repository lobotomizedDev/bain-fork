all:
	cargo build --release

add:
	cp -r images/* ~/.rain

clean:
	su -c 'rm -rf /usr/local/bin/rain'
	pkill rain

install: all
	su -c 'cp target/release/rain /usr/local/bin'
	mkdir -p ~/.rain
	cp -r images/* ~/.rain
	rain &

uninstall:
	cargo clean
	su -c 'rm -rf /usr/local/bin/rain'
	rm -rf ~/.rain
