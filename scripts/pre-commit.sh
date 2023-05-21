#!/bin/sh

if git rev-parse --verify HEAD >/dev/null 2>&1
then
	against=HEAD
else
	# Initial commit: diff against an empty tree object
	against=$(git hash-object -t tree /dev/null)
fi

# Redirect output to stderr.
exec 1>&2

if ! cargo fmt -- --check
then
    echo "There are some code style issues."
    echo "Run cargo fmt first."
    exit 1
fi

if ! cargo clippy --all-targets -- -D warnings
then
    echo "There are some clippy issues."
    exit 1
fi

# cargo sqlx prepare -- --lib
# git add sqlx-data.json

# If there are whitespace errors, print the offending file names and fail.
# exec git diff-index --check --cached "$against" --
