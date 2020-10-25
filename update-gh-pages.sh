#!/bin/sh -eu

repo_uri="https://x-access-token:${GITHUB_TOKEN}@github.com/${GITHUB_REPOSITORY}.git"
remote_name="origin"
main_branch="main"
target_branch="gh-pages"

cd "$GITHUB_WORKSPACE"

git config user.name "$GITHUB_ACTOR"
git config user.email "${GITHUB_ACTOR}@bots.github.com"

git checkout "$target_branch"
git reset --hard "${remote_name}/${main_branch}"

rustup target add wasm32-unknown-unknown

if which wasm-bindgen
then
  echo "wasm-bindgen-cli has already been installed"
else
  cargo install wasm-bindgen-cli
fi

./build.sh
git add -f "./pkg"

git commit -m "updates GitHub Pages"
if [ $? -ne 0 ]; then
    echo "nothing to commit"
    exit 0
fi

git remote set-url "$remote_name" "$repo_uri" # includes access token
git push --force-with-lease "$remote_name" "$target_branch"
