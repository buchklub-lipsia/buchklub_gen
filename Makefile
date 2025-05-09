.PHONY: b build deploy_website content_push

build:
	cargo run

preview: build
	open ../buchklub/index.html

reset_build:
	cd ../buchklub && git reset --hard

deploy: build
	cd ../buchklub && git add . && (date | xargs -0 git commit -m) && git push -f

content_push:
	git add content
	git commit -m "$(date | xargs echo content update)"
	git push

b: build
