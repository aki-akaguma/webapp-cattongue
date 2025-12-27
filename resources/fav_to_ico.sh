#!/bin/sh
# ImageMagick
convert +antialias -background transparent fav.svg -define icon:auto-resize=256,128,64,48,32,16 ../assets/favicon.ico
