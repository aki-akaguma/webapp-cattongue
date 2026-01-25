
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

clean:
	cargo clean

apply-patch:
	cargo patch-crate

bundle-web:
	dx bundle --web --release --base-path "/cattongue"

bundle-desktop:
	dx bundle --desktop --release

#	dx bundle --desktop --release --features backend_next

bundle-android-aarch64:
	@rm -fr "target/dx"
	dx bundle --android --release --target=aarch64-linux-android
	./scripts/apk-icon-assemble-r.sh cattongue aarch64 resources/android

bundle-android-x86_64:
	@rm -fr "target/dx"
	dx bundle --android --release --target=x86_64-linux-android
	./scripts/apk-icon-assemble-r.sh cattongue x86_64 resources/android

bundle-android-wv:
	@rm -fr "target/dx"
	./scripts/wv-apk-icon-assemble-r.sh cattongue resources/android ./scripts/android-webview-params.toml

bundle-android-wva:
	@rm -fr "target/dx"
	./scripts/wva-apk-icon-assemble-r.sh cattongue resources/android resources/android ./scripts/android-webview-assets-params.toml
