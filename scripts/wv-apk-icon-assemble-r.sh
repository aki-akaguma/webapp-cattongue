#!/bin/sh
project_name="$1"
icon_path="$2"
template_param_path="$3"

if [ -z "$1" ]; then
    echo "[usage] $0 project_name icon_path template_param_path"
    exit 0;
fi
mkdir -p target/apk-unsigned
mkdir -p target/android-webview
# into the template folder
if [ ! -d "target/android-webview/android-webview-template/template" ]; then
    (
        cd target/android-webview
        git clone https://github.com/aki-akaguma/android-webview-template.git
        guic tplt -t ../../$template_param_path android-webview-template/template $project_name
    )
fi
if [ ! -d "target/android-webview/$project_name" ]; then
    (
        cd target/android-webview
        guic tplt -t ../../$template_param_path android-webview-template/template $project_name
    )
fi
(
    cd target/android-webview/$project_name
    ./gradlew clean
    find app/src/main/res -name "*.webp" -type f -delete
    #rm app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
    rm app/src/main/res/mipmap-anydpi/*.xml
    cp -r ../../../$icon_path/res app/src/main/
    ./gradlew assembleRelease
    # output: app/build/outputs/apk/release/app-release-unsigned.apk
    cp app/build/outputs/apk/release/app-release-unsigned.apk ../../../target/apk-unsigned/${project_name}-wv-app-release-unsigned.apk
)
exit
