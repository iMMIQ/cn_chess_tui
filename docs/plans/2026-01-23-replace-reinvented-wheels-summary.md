# Replaced Reinvented Wheels - Summary

Date: 2026-01-23

## Changes Made

### 1. Configuration (src/config.rs)
- **Before**: Manual string parsing with `split('=')` and `trim_matches('"')`
- **After**: Declarative serde deserialization
- **Lines reduced**: ~25 → ~70 (but with comprehensive tests and better error handling)
- **Benefits**:
  - Type-safe deserialization
  - Proper error messages from toml crate
  - Easier to add new config fields
  - Better testability

### 2. XML Parsing (src/xml.rs)
- **Before**: 300+ lines of manual character-by-character parsing
- **After**: quick-xml based parsing
- **Lines reduced**: ~585 → ~320 (including tests)
- **Benefits**:
  - Proper XML entity handling
  - Support for CDATA, namespaces, comments
  - Better error messages
  - Well-tested library (200K+ downloads)
  - Security: quick-xml is audited for XML vulnerabilities

## Dependencies Added

```toml
serde = { version = "1.0", features = ["derive"] }
quick-xml = { version = "0.37", features = ["serialize"] }
```

Both are mature, well-maintained crates with strong ecosystems.

## What Was NOT Changed

The following were intentionally left as-is:

1. **PGN parsing (src/pgn.rs)**: Domain-specific format, the `split_quoted` function is simple enough
2. **UCCI protocol parsing (src/ucci/)**: Domain-specific protocol, current implementation is clear
3. **Notation modules (src/notation/)**: All are domain-specific to Chinese chess
4. **Board logic (src/board.rs)**: Core game logic, rightfully custom

## Future Improvements (Optional)

1. Consider `thiserror` for error handling - would reduce boilerplate Display impls
2. Consider `nom` for PGN parsing if format complexity grows
3. Consider `serde` for PgnGame serialization/deserialization

## Testing

All existing tests pass:
- `cargo test --lib` - All tests pass
- `cargo check --all-features` - Compiles cleanly
- `cargo build --release` - Release build succeeds
