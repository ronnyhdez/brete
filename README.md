# brete

CLI to log project working hours

`brete` is a small terminal timer: start it with a project and tag, stop it when you're done, and it records the session to a plain CSV file. No servers, no databases, no environment to set up — just a single compiled binary.

## Install

Requires [Rust/Cargo](https://rustup.rs/).

```sh
git clone <this-repo-url> brete_logger
cd brete_logger
```

If `~/.cargo/bin` is already on your `PATH` (the default after installing Rust via [rustup](https://rustup.rs/)), the easiest way to install is:

```sh
cargo install --path .
```

This builds in release mode and drops the binary straight into `~/.cargo/bin` — no `sudo` needed.

Otherwise, build manually and copy the binary onto your `PATH` (requires sudo since `/usr/local/bin` is root-owned):

```sh
cargo build --release
sudo cp target/release/brete /usr/local/bin/brete
sudo chmod 755 /usr/local/bin/brete
```

Verify it's installed:

```sh
brete status
# No timer running.
```

## Usage

### Start a timer

```sh
brete start <project> [tag...]
```

The first word is the project; everything after it is joined into the tag. If no tag is given, it defaults to `general`.

```sh
brete start openscapes core
# Started openscapes/core at 10:04
```

Only one timer can run at a time — starting a second one while one is active will error out and tell you what's currently running.

### Check what's running

```sh
brete status
# Running: openscapes/core since 10:04 (12m elapsed)
```

### Stop the timer

```sh
brete stop
# Stopped openscapes/core — 12m
```

This appends the completed session to the log and clears the running timer.

### View past sessions

```sh
brete log
```

Prints all recorded sessions (most recent first) as a table: project, tag, duration, start, and end time.

## Data storage

Sessions are stored as plain CSV in `~/.brete/log.csv`, one row per completed session (`start,end,duration_minutes,project,tag`). It's human-readable and safe to open directly, back up, or import into a spreadsheet.

The currently running timer (if any) is kept in `~/.brete/current.json` and removed once you run `brete stop`.
