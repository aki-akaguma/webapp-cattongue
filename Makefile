
all: list

MAKEFILE_LIST = Makefile
# Self-documenting Makefile targets script from Stack Overflow
# Targets with comments on the same line will be listed.
list:
	@LC_ALL=C $(MAKE) -pRrq -f $(firstword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/(^|\n)# Files(\n|$$)/,/(^|\n)# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: list

bundle-web:
	dx bundle --web --release --base-path "/broinfo"

bundle-desktop:
	dx bundle --desktop --release

#	dx bundle --desktop --release --features backend_next

bundle-android:
	dx bundle --android --release --target=aarch64-linux-android

