This repository serves as a guide for FE/BE structure using rust.

To start:
```
mkdir <project_dir>
cargo new --bin server --vcs none
cargo new --bin frontend --vcs none
echo -e '[workspace]\nmembers = ["server", "frontend"]' > Cargo.toml
echo -e "target/\ndist/" > .gitignore
git init
```

After that you can apply patch accordingly.
