#!/bin/bash

shopt -s extglob

rm assets/!(*.gitignore|*.rs|*.ttf)

find ../QuestHearthAssets -name \*.zip -exec cp {} assets/ \;

cd assets/

unzip -o '*.zip'
rm *.zip

for dir in */
do
    rm -rf $dir &
done

cd ..
