.PHONY: setup clean

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

