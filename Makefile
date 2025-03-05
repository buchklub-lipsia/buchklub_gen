.PHONY: b build deploy_website

build:
	cargo run

deploy: build
	cd ../buchklub && git add . && (date | xargs -0 git commit -m) && git pull && git push

b: build
