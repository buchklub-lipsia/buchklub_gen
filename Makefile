.PHONY: build deploy_website

.ONESHELL:

deploy_website: build
	cd ../buchklub
	git add .
	date | xargs -0 git commit -m
	git pull
	git push

build:
	cargo r
