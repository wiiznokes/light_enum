test:
	clear && cargo test --all --all-features

expand:
	clear && cargo expand --bin main


build:
	clear && cargo build --release

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

clean-git:
	git rm -rf --cached .
	git add .