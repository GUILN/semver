CT=cargo test
CD=cargo doc --no-deps --open
CR=cargo build --release
CF=cargo fmt

test:
	cd ./core && $(CT)

doc:
	$(CD)

release: test
	$(CR)

format:
	$(CF)

