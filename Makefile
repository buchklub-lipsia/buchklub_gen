.PHONY: b build deploy_website content_push

build:
	cargo run

deploy: build
	cd ../buchklub && git add . && (date | xargs -0 git commit -m) && git push -f

content_push:
	git add content
	git commit -m "$(date | xargs echo content update)"
	git push

b: build
