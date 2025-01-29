.ONESHELL:

deploy_website: build
	cd ../buchklub
	git add .
	date | xargs -0 git commit -m
	git push

build:
	cd ../buchklub_inhalte
	git switch master
	git pull
	make
	cd ../buchklub_gen
	git pull
	cargo r
