#!/bin/sh
project_name="$1"
arch="$2"
icon_path="$3"

if [ -z "$1" ]; then
    echo "[usage] $0 project_name arch icon_path"
    exit 0;
fi
mkdir -p target/apk-unsigned
# into the release folder
(
    cd target/dx/$project_name/release/android/app/
    # do clean and replace the icons, then build
    ./gradlew clean
    find app/src/main/res -name "*.webp" -type f -delete
    rm app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
    cp -r ../../../../../../$icon_path/res app/src/main/
    ./gradlew assembleRelease
    # output: app/build/outputs/apk/release/app-release-unsigned.apk
    cp app/build/outputs/apk/release/app-release-unsigned.apk ../../../../../../target/apk-unsigned/${project_name}-${arch}-app-release-unsigned.apk
)
exit
