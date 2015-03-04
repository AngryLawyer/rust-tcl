#!/bin/bash

rev=$(git rev-parse --short HEAD)

if [ ! -d "target/doc" ]; then
    echo "Docs aren't built - aborting"
    exit -1;
fi

cd target/doc

git init

git remote add upstream "git@github.com:AngryLawyer/rust-tcl.git"
git fetch upstream && git reset upstream/gh-pages

touch .

git add -A .
git commit -m "rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
