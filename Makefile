CT=cargo test
CD=cargo doc --no-deps --open

test:
	cd ./core && $(CT)

doc:
	$(CD)

