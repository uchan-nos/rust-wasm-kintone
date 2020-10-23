#!/bin/sh -eu

repo_uri="https://x-access-token:${GITHUB_TOKEN}@github.com/${GITHUB_REPOSITORY}.git"
remote_name="origin"
main_branch="main"
target_branch="gh-pages"

cd "$GITHUB_WORKSPACE"

git config user.name "$GITHUB_ACTOR"
git config user.email "${GITHUB_ACTOR}@bots.github.com"

git checkout "$target_branch"
git rebase "${remote_name}/${main_branch}"

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

./build.sh
git add "./pkg"

git commit -m "updated GitHub Pages"
if [ $? -ne 0 ]; then
    echo "nothing to commit"
    exit 0
fi

git remote set-url "$remote_name" "$repo_uri" # includes access token
git push --force-with-lease "$remote_name" "$target_branch"
