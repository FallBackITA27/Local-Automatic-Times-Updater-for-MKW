rm json
rm .gitignore
rm cargo*
rm current_version
rm readme.md
mkdir ./releases
cargo build --release
mv ./target/release/LOCMKWUPD ./releases/LOCMKWUPD