if [ -z "$BLAZE_PATH" ]
then
    echo 'BLAZE_PATH not set, make sure that blaze library is in linker path!'
    echo 'Visit https://github.com/razer-rbi/blaze for building instructions'
fi

OLD_LD=$LD_LIBRARY_PATH
export LD_LIBRARY_PATH=$BLAZE_PATH

cargo tarpaulin -v --exclude-files tests/* --out Html
mkdir docs/coverage -p
mv tarpaulin-report.html docs/coverage/index.html

cargo doc --no-deps
rm -rf docs/api/*
mkdir docs/api -p
cp -r target/doc/* docs/api/

export LD_LIBRARY_PATH=$OLD_LD
