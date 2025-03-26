#!/usr/bin/env bash

# Run the game of life for the given language and arguments

set -eu

if [ $# -lt 1 ]; then
    echo "Usage: $0 <language> [args...]"
    echo "Supported languages: lua, luajit, rust"
    exit 1
fi

LANG=$1
shift  # Remove the language argument, leaving only the additional args

case $LANG in
    "lua")
        exec lua langs/lua/life.lua "$@"
        ;;
    "luajit")
        exec luajit langs/lua/life.lua "$@"
        ;;
    "rust")
        exec target/release/life "$@"
        ;;
    *)
        echo "Unsupported language: $LANG"
        echo "Supported languages: lua, luajit, rust"
        exit 1
        ;;
esac
