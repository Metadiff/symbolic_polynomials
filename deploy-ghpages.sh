#!/bin/bash

cargo doc
(cd target/doc
git init
git config user.name "botev"
git config user.email "botevmg@gmail.com"
cp ../../README.md ./README.md
cp ../../LICENSE ./LICENSE
cp ../../index.html ./index.html
git add .
git commit -m "Deployed to Github Pages"
git push --force --quiet "https://${GH_TOKEN}@${GH_REF}" master:gh-pages)
