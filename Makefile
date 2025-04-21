.PHONY: setup build run check-env

# Check if the libtorch directory exists
check-env:
	@if [ ! -d "libtorch" ]; then \
		echo "libtorch directory does not exist. Please run 'make setup' first."; \
		exit 1; \
	fi

setup:
	mkdir -p libtorch; \
		echo "Downloading libtorch..."; \
		curl -L https://download.pytorch.org/libtorch/cpu/libtorch-macos-arm64-2.6.0.zip -o libtorch.zip; \
		unzip libtorch.zip -d .; \
		rm libtorch.zip; \
		export LIBTORCH=$(PWD)/libtorch; \
		echo "LIBTORCH set to $$LIBTORCH"; \

build: check-env
	LIBTORCH=$(PWD)/libtorch DYLD_LIBRARY_PATH=$(PWD)/libtorch/lib cargo build

run: check-env
	LIBTORCH=$(PWD)/libtorch DYLD_LIBRARY_PATH=$(PWD)/libtorch/lib cargo run

clean:
	rm -rf libtorch libtorch.zip
	cargo clean

