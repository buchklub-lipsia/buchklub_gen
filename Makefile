.PHONY: b build deploy_website content_push verify fmt

build:
	cargo run

verify:
	for f in $$(find . -name "*.gon"); do \
		echo -n "$$f " && gon verify $$f ; \
	done

fmt: verify
	for f in $$(find . -name "*.gon"); do \
		echo "Formatting $$f" ; \
		gon fmt -w 4 -t -i -m 80 $$f ; \
	done

preview: build
	open ../buchklub/index.html | firefox ../buchklub/index.html

reset_build:
	cd ../buchklub && git reset --hard

deploy: build
	cd ../buchklub && git add . && (date | xargs -0 git commit -m) ; git push -f

content_push: fmt
	git add content
	git commit -m "$$(date | xargs echo content update)" || echo nothing to commit
	git push

b: build
