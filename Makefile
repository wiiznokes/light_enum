

expand:
	clear && cargo expand --bin test

release:
	clear && cargo run --release

run:
	clear && cargo run

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

clean-git:
	git rm -rf --cached .
	git add .