
run:
	cargo run cards.txt

build:
	cargo build

windows:
	cargo build --target=x86_64-pc-windows-gnu

push-tags:
	git push --tags

maybe-better-git-push:
	git push --follow-tags

