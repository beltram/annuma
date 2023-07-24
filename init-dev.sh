#!/bin/zsh

cargo install --path .
sudo annuma completion

export ZSH_DISABLE_COMPFIX=true
echo "✅ annuma is ready"

exec zsh