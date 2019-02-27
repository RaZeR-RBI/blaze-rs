cargo tarpaulin -v --exclude-files tests/* --out Html
mkdir docs/coverage -p
mv tarpaulin-report.html docs/coverage/index.html
