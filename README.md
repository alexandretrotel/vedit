# vedit

`vedit` is a small Rust command-line wrapper that runs `auto-editor` to remove silent parts from a video and then uses `ffmpeg` to adjust playback speed. It automates the two-step process: run `auto-editor` with a configurable margin, then apply video and audio speed filters with `ffmpeg`.

This repository contains a minimal CLI (see `src/main.rs`) that expects `auto-editor` and `ffmpeg` to be available on your PATH.

## Requirements

- Rust (stable) — to build the project
- `auto-editor` — installed and available on your PATH
- `ffmpeg` — installed and available on your PATH

Note: `vedit` assumes that after running:
`auto-editor <input> --margin <N>sec`
an altered file will exist with the same stem as the input and the suffix `_ALTERED` (e.g. `video.mp4` -> `video_ALTERED.mp4`). Ensure your `auto-editor` installation/version produces that output or configure/move the file accordingly.

## Building

From cargo:

```bash
cargo install vedit
```

From the project root:

```bash
# Build in release mode, binary will be in ~/.cargo/bin
cargo install --path .
```

## Usage

Basic invocation:

```bash
vedit <input> --margin <seconds> --speed <factor> --output <output>
```

Arguments:

- `<input>` — Path to the input video file (positional).
- `--margin` — Margin in seconds passed to `auto-editor` (float, e.g. `0.2`).
- `--speed` — Playback speed factor applied with `ffmpeg` (float, e.g. `1.25`, `1.5`, `2.0`).
- `--output` — Output filename for the final processed video.

Example:

```bash
# Remove silent parts (with 0.2s margin) then speed playback 1.5x, writing to output.mp4
vedit input.mp4 --margin 0.2 --speed 1.5 --output output.mp4
```

## How it works

The CLI performs the following steps:

1. Parses arguments using `clap`.
2. Runs:
   - `auto-editor <input> --margin <margin>sec`
   - Expects an altered file named `<stem>_ALTERED.<ext>` to exist after that command.
3. Runs `ffmpeg` on the altered file to change speed:
   - Video filter: `setpts=PTS/<speed>`
   - Audio filter: `atempo=<speed>`
4. Writes the final file to the path provided by `--output`.

The code returns errors when the external commands exit with non-zero status:

- If `auto-editor` fails, the program reports `auto-editor failed`.
- If `ffmpeg` fails, the program reports `ffmpeg failed`.

## Limitations and caveats

- `atempo` accepts only certain ranges per filter invocation (commonly 0.5–2.0). If you pass a `--speed` outside ffmpeg's single-step `atempo` range, `ffmpeg` may fail. A robust solution would chain multiple `atempo` filters for larger speed multipliers (not implemented here).
- The tool relies on `auto-editor` producing an altered file with the `_ALTERED` suffix. If your `auto-editor` version uses a different naming pattern, you'll need to rename the altered file before `vedit` runs `ffmpeg`, or modify the code.
- This is a thin wrapper: all heavy lifting is done by `auto-editor` and `ffmpeg`. `vedit` does not attempt to parse or modify media streams itself.

## Contributing

This project is intentionally small. If you want to:

- Support chaining `atempo` for arbitrary speeds,
- Make the altered filename configurable,
- Add better logging or dry-run mode,

feel free to open a PR or modify `src/main.rs`.
