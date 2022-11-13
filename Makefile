CT=cargo test
CD=cargo doc --no-deps --open
CR=cargo build --release

test:
	cd ./core && $(CT)

doc:
	$(CD)

release:
	$(CR)

