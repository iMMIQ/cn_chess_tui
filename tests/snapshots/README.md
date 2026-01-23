# UI Snapshots

This directory contains golden UI snapshots managed by [insta](https://insta.rs/).

## File Naming

Snapshots are auto-named by insta: `{test_file}__{test_function}.snap`

Example: `ui_snapshots__initial_position_ui.snap`

## DO NOT edit manually

Snapshots are managed by `cargo-insta`. To update:

```bash
cargo test --test ui_snapshots
cargo insta review
```

## Current Coverage

- Initial position (3 sizes: small 40x26, standard 80x24, large 120x40)
- Mid-game states (after first move)
- Check and checkmate states
- Layout modes (compact 30x24, standard 60x30, full 100x40)

**Total snapshots**: 9

## Snapshot Files

- `ui_snapshots__initial_position_ui.snap` - Standard 80x24 initial position
- `ui_snapshots__initial_position_small_terminal.snap` - Compact 40x26 initial position
- `ui_snapshots__initial_position_large_terminal.snap` - Spacious 120x40 initial position
- `ui_snapshots__after_first_move.snap` - Game state after opening move
- `ui_snapshots__check_state.snap` - When a general is in check
- `ui_snapshots__checkmate_state.snap` - Game ending in checkmate
- `ui_snapshots__compact_layout.snap` - Minimal 30x24 layout
- `ui_snapshots__standard_layout.snap` - Balanced 60x30 layout
- `ui_snapshots__full_layout.snap` - Spacious 100x40 layout
