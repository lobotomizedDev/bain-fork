all:
	cargo build --release

add:
	cp -r images/* ~/.rain/images

clean:
	su -c 'rm -rf /usr/local/bin/rain'
	pkill rain

install: all
	su -c 'cp target/release/rain /usr/local/bin'
	mkdir -p ~/.rain/images
	cp -r images/* ~/.rain/images
	rain &

uninstall:
	cargo clean
	su -c 'rm -rf /usr/local/bin/rain'
	rm -rf ~/.rain
