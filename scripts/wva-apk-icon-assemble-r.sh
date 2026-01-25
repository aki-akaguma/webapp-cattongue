#!/bin/sh
project_name="$1"
icon_path="$2"
assets_path="$3"
template_param_path="$4"

if [ -z "$1" ]; then
    echo "[usage] $0 project_name icon_path assets_path template_param_path"
    exit 0;
fi
mkdir -p target/apk-unsigned
mkdir -p target/android-webview-assets
# into the template folder
if [ ! -d "target/android-webview-assets/android-webview-assets-template/template" ]; then
    (
        cd target/android-webview-assets
        git clone https://github.com/aki-akaguma/android-webview-assets-template.git
        guic tplt -t ../../$template_param_path android-webview-assets-template/template $project_name
    )
fi
if [ ! -d "target/android-webview-assets/$project_name" ]; then
    (
        cd target/android-webview-assets
        guic tplt -t ../../$template_param_path android-webview-assets-template/template $project_name
    )
fi
(
    cd target/android-webview-assets/$project_name
    ./gradlew clean
    find app/src/main/res -name "*.webp" -type f -delete
    #rm app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
    rm app/src/main/res/mipmap-anydpi/*.xml
    cp -r ../../../$icon_path/res app/src/main/
    cp -r ../../../$assets_path/assets app/src/main/
    ./gradlew assembleRelease
    # output: app/build/outputs/apk/release/app-release-unsigned.apk
    cp app/build/outputs/apk/release/app-release-unsigned.apk ../../../target/apk-unsigned/${project_name}-wva-app-release-unsigned.apk
)
exit
