# This script takes care of testing your crate

set -ex

main() {
    # cross build --target $TARGET
    # cross build --target $TARGET --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi
    if [ -z $TEST_DIR ]; then
        cd $TEST_DIR
    fi
    # cross test --target $TARGET
    # cross test --target $TARGET --release
    cargo test

}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
