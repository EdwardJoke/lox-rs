cd ..
cargo clean
cargo build --release
cd target/release

version=$(cargo metadata --format-version 1 | jq -r '.packages[0].version')

tar -czf lox-darwin-amd64-v$version.tar.gz lox
