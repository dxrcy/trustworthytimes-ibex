# Watch and compile for development, open a local http server
serve:
	cargo watch -x 'run -- local' -i build &\
	basic-http-server build

# Install all dependencies
install:
	cargo install cargo-watch basic-http-server

