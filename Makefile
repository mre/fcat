.PHONY: build
build:
	docker build -t fcat .

.PHONY: run
run:
	# Use init for signal handling
	# Privileged required for mounting the ramdisk
	docker run -it --init --privileged fcat

.PHONY: sh
sh:
	docker run -v ${PWD}:/fcat -it --privileged --entrypoint bash fcat
