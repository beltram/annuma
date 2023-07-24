#!/bin/zsh

cargo install --force --git https://github.com/beltram/annuma.git
sudo annuma completion

export ZSH_DISABLE_COMPFIX=true

echo "✅ annuma is installed"
exec zsh