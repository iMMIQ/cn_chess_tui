# UI Snapshot Testing Guide

## Overview

This project uses [insta](https://github.com/mitsuhiko/insta) for snapshot testing. Snapshots capture the exact terminal UI output and compare against stored "golden" snapshots to detect unintended changes.

## Why insta?

- **Minimal code**: ~50 lines vs ~500 lines for custom implementation
- **Interactive review**: `cargo insta review` for approving changes
- **Great diffs**: Automatic color-coded diff output
- **Industry standard**: Used by Rust compiler (19,000+ snapshots)
- **Zero maintenance**: No need to maintain custom snapshot code

## Quick Start

### Installation

```bash
# Install CLI tool (one-time)
cargo install cargo-insta

# Add to project (already done)
cargo add insta --dev
```

### Running Tests

```bash
# Run all snapshot tests
cargo test --test ui_snapshots

# Run specific test
cargo test test_initial_position_ui

# Run with output
cargo test --test ui_snapshots -- --nocapture
```

### Managing Snapshots

```bash
# Review pending snapshot changes interactively
cargo insta review

# Accept all new/changed snapshots
cargo insta accept

# Reject all pending changes
cargo insta reject

# Run tests and automatically review
cargo insta test --review
```

## Creating New Snapshot Tests

Add test to `tests/ui_snapshots.rs`:

```rust
#[test]
fn test_my_game_state() {
    let game = Game::from_fen("your/fen/string").unwrap();
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();

    terminal.draw(|f| {
        let ui = GameUI::new(game.clone());
        ui.render(f, f.area());
    }).unwrap();

    assert_snapshot!(terminal.backend());
}
```

Run test:

```bash
cargo test test_my_game_state
cargo insta accept
```

## Understanding Test Failures

When a snapshot test fails:

```bash
cargo test test_initial_position_ui
```

Output:

```
---- test_initial_position_ui stdout ----
Snapshot file: tests/snapshots/ui_snapshots__initial_position_ui.snap
Context: ui_snapshots

-insta: snapshot mismatch for "test_initial_position_ui"
───────────────────────
-   │─┬─┬────────────┬──┬──┬─────
+   │─┬─┬────────────┬──┬──┬────▒
    │红│黑│楚河 汉界   │红│黑│
```

### If change is intentional:

```bash
# Review interactively
cargo insta review

# Or accept directly
cargo insta accept

# Commit both code and snapshot
git add src/ tests/snapshots/
git commit -m "feat: update UI layout"
```

### If change is unintentional:

Investigate the regression in your code and fix it.

## Snapshot File Format

insta snapshot files are stored in `tests/snapshots/`:

```rust
---
source: tests/ui_snapshots.rs
expression: terminal.backend()
---
"│─┬─┬────────────┬──┬──┬─────\n"
"│红│黑│楚河 汉界   │红│黑│    \n"
"├─┼─┼────────────┼──┼──┼─────\n"
...
```

The `.snap` files are automatically managed by insta.

## Best Practices

1. **Descriptive test names**: `test_check_state` not `test_state_1`
2. **One snapshot per logical state**: Test key game states, not every position
3. **Include edge cases**: Initial position, check, checkmate, different sizes
4. **Review before committing**: Always review snapshot changes
5. **Commit snapshots with code**: Snapshots are part of the test suite
6. **Update docs**: Document complex game states in comments

## CI/CD

Snapshot tests run automatically on PRs in GitHub Actions.

If tests fail in CI:
1. Check the workflow logs for snapshot diffs
2. If change is intentional: run `cargo insta accept` locally and commit
3. If unintentional: fix the code

## Troubleshooting

### Snapshot not found

**Error**: "snapshot file not found"

**Solution**: Run `cargo insta accept` to create initial snapshot

### All tests fail after refactor

**Solution**:
```bash
# Review all changes at once
cargo insta review

# Or accept all if refactor is intentional
cargo insta accept
```

### Flaky tests

**Cause**: Non-deterministic game state or timing

**Solution**: Ensure game state is fully deterministic before rendering

### Encoding issues

**Symptom**: Garbled characters in snapshots

**Solution**: Set locale to UTF-8:
```bash
export LANG=en_US.UTF-8
cargo test
```

## Advanced Usage

### Named snapshots

```rust
assert_snapshot!("my_custom_name", terminal.backend());
```

### Multiple snapshots in one test

```rust
assert_snapshot!(snapshot_id = "state_1", terminal.backend());
// ... modify game ...
assert_snapshot!(snapshot_id = "state_2", terminal.backend());
```

### Inline snapshots (for small outputs)

```rust
assert_snapshot!(my_output, @"expected output");
```

## Current Test Coverage

- Initial position (3 sizes: 40x26, 80x24, 120x40)
- Mid-game states (after first move)
- Check and checkmate states
- Compact, standard, and full layouts
- Multiple terminal dimensions

**Total snapshots**: 9

## Resources

- [insta documentation](https://insta.rs/)
- [Ratatui snapshot testing recipe](https://ratatui.rs/recipes/testing/snapshots/)
- [cargo-insta CLI reference](https://github.com/mitsuhiko/insta/blob/main/cargo-insta/README.md)
