#!/usr/local/bin/fish

if test (count $argv) -lt 1
    echo "./setup.sh <id-text-name>"
    exit
end

cargo new --vcs none --name cses $argv[1]
