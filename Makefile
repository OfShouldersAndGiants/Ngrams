.DEFAULT_GOAL := help
.PHONY: help setup clean

help:
	@echo "Usage: make [target]"
	@echo "Targets:"
	@echo "  setup  - Download and set up libtorch (macOS arm64) and set LIBTORCH"
	@echo "  clean  - Remove libtorch artifacts and run cargo clean"

setup:
	mkdir -p libtorch; \
		echo "Downloading libtorch..."; \
		curl -L https://download.pytorch.org/libtorch/cpu/libtorch-macos-arm64-2.6.0.zip -o libtorch.zip; \
		unzip libtorch.zip -d .; \
		rm libtorch.zip; \
		export LIBTORCH=$(PWD)/libtorch; \
		echo "LIBTORCH set to $$LIBTORCH"; \

clean:
	rm -rf libtorch libtorch.zip
	cargo clean

