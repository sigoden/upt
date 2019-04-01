# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage= \
          targets=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            targets=(i686-unknown-linux-musl x86_64-unknown-linux-musl)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            targets=(x86_64-apple-darwin)
            ;;
    esac

    for target in ${targets[@]}; do
        rustup target add $target

        cargo rustc --bin $CRATE_NAME --target $target --release -- -C lto

        cp target/$target/release/$CRATE_NAME $stage/

        cd $stage
        tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$target.tar.gz *
        cd $src
    done

    rm -rf $stage
}

main
