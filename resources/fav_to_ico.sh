#!/bin/sh
# ImageMagick
#convert +antialias -background transparent fav.svg -define icon:auto-resize=256,128,64,48,32,16 ../assets/favicon.ico

resvg -w 256 -h 256 fav.svg ../assets/app.png
convert +antialias -background transparent ../assets/app.png -define icon:auto-resize=256,128,64,48,32,16 ../assets/favicon.ico
