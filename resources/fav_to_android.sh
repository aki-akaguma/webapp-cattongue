#!/bin/sh
#
convert +antialias -background transparent fav.svg -resize 192x192 android/res/mipmap-xxxhdpi/ic_launcher.webp
convert +antialias -background transparent fav.svg -resize 144x144 android/res/mipmap-xxhdpi/ic_launcher.webp
convert +antialias -background transparent fav.svg -resize 96x96 android/res/mipmap-xhdpi/ic_launcher.webp
convert +antialias -background transparent fav.svg -resize 72x72 android/res/mipmap-hdpi/ic_launcher.webp
convert +antialias -background transparent fav.svg -resize 48x48 android/res/mipmap-mdpi/ic_launcher.webp
