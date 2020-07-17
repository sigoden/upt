# This script takes care of building your crate and packaging it for release

set -ex

build() {
    local src=$(pwd) \
        stage=$1 \
        TARGET=$2 \

    rustup target add $TARGET

    cargo rustc --bin $CRATE_NAME --target $TARGET --release -- -C lto

    cp target/$TARGET/release/$CRATE_NAME $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage/*
}

case $TRAVIS_OS_NAME in
    linux)
        stage=$(mktemp -d)
        build $stage i686-unknown-linux-musl
        build $stage x86_64-unknown-linux-musl
        ;;
    osx)
        stage=$(mktemp -d -t tmp)
        build $stage x86_64-apple-darwin
        ;;
esac
