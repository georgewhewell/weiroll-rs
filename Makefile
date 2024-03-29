bindings: rustfmt

rustfmt: src/bindings
	rustfmt src/bindings/*.rs

src/bindings: weiroll/node_modules
	forge bind \
		--hardhat \
		--root weiroll \
		--module \
		--bindings-path ./src/bindings \
		--select-all

weiroll/node_modules:
	cd weiroll && npm install

clean:
	rm -rf ./src/bindings

.PHONY: bindings clean rustfmt