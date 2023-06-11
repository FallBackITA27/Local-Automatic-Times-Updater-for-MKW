Remove-Item ./json -Recurse
Remove-Item ./.gitignore
Remove-Item ./cargo.*
Remove-Item ./current_version
Remove-Item ./readme.md
$path = "./releases"
If(!(test-path -PathType container $path))
{
      New-Item -ItemType Directory -Path $path
}
cargo build --release
Move-Item ./target/release/LOCMKWUPD ./releases/LOCMKWUPD