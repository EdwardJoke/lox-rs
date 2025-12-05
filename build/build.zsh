cd ..
cargo clean
cargo build --release
cd target/release

version=$(cargo pkgid | cut -d '@' -f 2)

tar -czf lox-darwin-amd64-v$version.tar.gz lox