# Releasing the crates

Bump the version:
```
find . -name Cargo.toml -exec sed -i "s/<current-version>/<next-version>/gm" {} +
cargo build
```

Publish:
```
make publish
```
