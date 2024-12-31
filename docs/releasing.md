# Releasing the crates

Bump the version:
```
find . -name Cargo.toml -exec sed -i "s/<current-version>/<next-version>/gm" {} +
cargo build
```

Git commit changes to Cargo.toml and Cargo.lock files.

Publish:
```
make publish
```

Git push.
