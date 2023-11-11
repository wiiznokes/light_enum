test:
	clear && cargo test --all --all-features

expand-light-enum:
	clear && cargo expand --test light_enum

expand-values:
	clear && cargo expand --test values

build:
	clear && cargo build --release

fix:
	cargo clippy --all --fix --allow-dirty --allow-staged
	cargo fmt --all

clean-git:
	git rm -rf --cached .
	git add .