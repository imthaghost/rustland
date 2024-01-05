SHELL := /bin/bash



clean:
	# remove python build
	rm -rf client/bin client/include client/lib
	# remove rust build
	rm -rf target

install:
	# python dependencies
	pip3 install -r client/requirements.txt
	# rust dependencies
	cargo build --release

receiver:
	# run client
	python3 client/named.py

sender:
	# run sender
	cargo run --release


