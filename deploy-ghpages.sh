#!/bin/bash
if [[ "$TRAVIS_RUST_VERSION" == "stable" && "$TRAVIS_BRANCH" == "master" ]]; then
    cargo doc
    cd target/doc
    git init
    git checkout -b gh-pages
    git config user.name "botev"
    git config user.email "botevmg@gmail.com"
    cp ../../README.md ./README.md
    cp ../../LICENSE ./LICENSE
    cp ../../index.html ./index.html
    git add .
    git commit -m "Deployed to Github Pages"
    git push -f -q https://botev:${GITHUB_API_KEY}@${GH_REF} gh-pages 1>&2 2>/dev/null
    echo "Deployed to ghpages branch"
    cd ${TRAVIS_BUILD_DIR}
fi
