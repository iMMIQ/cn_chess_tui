# Chinese Chess TUI

A terminal-based (TUI) implementation of Chinese Chess (Xiangqi) written in Rust using the Ratatui framework.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

Chinese Chess TUI is a fully-featured implementation of the classic Chinese board game Xiangqi for the terminal. Play against a friend locally on the same computer with an intuitive text-based interface.

## Features

- Complete implementation of Chinese Chess rules
- Interactive terminal user interface with keyboard controls
- Visual board with Chinese characters for pieces
- Move validation for all piece types:
  - General (Shuai/Jiang) - Palace movement, flying general rule
  - Advisor (Shi) - Diagonal movement within palace
  - Elephant (Xiang) - Diagonal 2-square movement, cannot cross river
  - Horse (Ma) - L-shaped movement with hobbling rule
  - Chariot (Ju) - Orthogonal movement like a rook
  - Cannon (Pao) - Moves like chariot, captures by jumping over one piece
  - Soldier (Bing/Zu) - Forward movement, sideways after crossing river
- Check and checkmate detection
- Move history tracking
- Undo functionality
- Game restart capability

## Controls

| Key | Action |
|-----|--------|
| `Arrow Keys` | Move cursor |
| `Enter` | Select piece / Confirm move |
| `u` | Undo last move |
| `r` | Restart game |
| `q` / `Esc` | Quit game |

## How to Play

1. Red moves first
2. Use arrow keys to navigate to your piece
3. Press `Enter` to select the piece
4. Navigate to the destination square and press `Enter` to move
5. Take turns with your opponent until checkmate or stalemate

## Installation

### Prerequisites

- Rust 1.70 or later
- A terminal that supports UTF-8 and alternative screen mode

### Building from Source

```bash
git clone <repository-url>
cd cn_chess_tui
cargo build --release
```

The compiled binary will be available at `target/release/cn_chess_tui`.

## Running

```bash
cargo run --release
```

Or directly:

```bash
./target/release/cn_chess_tui
```

## Piece Characters

| Piece | Red | Black |
|-------|-----|-------|
| General | 帅 | 将 |
| Advisor | 仕 | 士 |
| Elephant | 相 | 象 |
| Horse | 马 | 马 |
| Chariot | 车 | 车 |
| Cannon | 炮 | 炮 |
| Soldier | 兵 | 卒 |

## Rules Summary

- **General**: Moves one point orthogonally within the palace (3x3 area)
- **Advisor**: Moves one point diagonally within the palace
- **Elephant**: Moves exactly two points diagonally, cannot cross the river, can be blocked
- **Horse**: Moves one point orthogonally then one point diagonally, can be blocked
- **Chariot**: Moves any distance orthogonally like a rook in Western chess
- **Cannon**: Moves like a chariot but must jump over exactly one piece to capture
- **Soldier**: Moves one point forward; after crossing the river, can also move sideways

### Special Rules

- **Flying General**: The two generals cannot face each other on the same file with no pieces between them
- **Check**: A general is in check when it could be captured on the opponent's next turn
- **Checkmate**: The game ends when a player's general is in check with no legal moves to escape
- **Stalemate**: The game is a draw if a player has no legal moves but is not in check

## Testing

Run the test suite:

```bash
cargo test
```

Tests cover:
- Initial position setup
- Soldier movement (forward, sideways, backward restrictions)
- Flying general rule

## Future Features

Potential enhancements for future versions:

- [ ] AI opponent with configurable difficulty
- [ ] Save/Load game functionality
- [ ] Move notation and PGN export
- [ ] Replay mode for reviewing games
- [ ] Timer for timed games
- [ ] Network play for online matches
- [ ] Custom board themes and colors
- [ ] Sound effects for moves and captures

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui) - A Rust library for building terminal user interfaces
- Inspired by the ancient game of Xiangqi (Chinese Chess)
