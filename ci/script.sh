#!/usr/bin/env bash
set -ex

cargo=cargo
target_param=""
if [ ! -z "$TARGET" ]; then
    rustup target add "$TARGET"
    cargo install -v cross --force
    cargo="cross"
    target_param="--target $TARGET"
fi

$cargo build -v $target_param

$cargo test -v $target_param

$cargo bench -v $target_param -- --test # don't actually record numbers

$cargo doc -v $target_param

$cargo test -v --release

if [ ! -z "$COVERAGE" ]; then
    if [ ! -z "$TARGET" ]; then
        echo "cannot record coverage while cross compiling"
        exit 1
    fi

    cargo install -v cargo-travis || echo "cargo-travis already installed"
    cargo coverage -v -m coverage-reports $features_param --kcov-build-location "$PWD/target"
    bash <(curl -s https://codecov.io/bash) -c -X gcov -X coveragepy -s coverage-reports
fi


if [ ! -z "$FUZZ" ]; then
    if [ ! -z "$TARGET" ]; then
        echo "cannot fuzz while cross compiling"
        exit 1
    fi

    ./fuzzit.sh local-regression

    branch=$(if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then echo $TRAVIS_BRANCH; else echo $TRAVIS_PULL_REQUEST_BRANCH; fi)
    if [ "$branch" = "master" ]; then
        # a build on master, so let's update the long-running jobs
        ./fuzzit.sh fuzzing
    fi
fi
