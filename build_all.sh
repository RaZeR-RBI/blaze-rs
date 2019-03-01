cargo tarpaulin -v --exclude-files tests/* --out Html
mkdir docs/coverage -p
mv tarpaulin-report.html docs/coverage/index.html

cargo doc --no-deps
rm -rf docs/api/*
mkdir docs/api -p
cp -r target/doc/* docs/api/
