# Clipboard History Manager

A simple clipboard history manager written in Rust that continuously listens for clipboard changes and logs them to a file. It supports automatic trimming of history, background saving, and runs efficiently with minimal CPU usage.

## Features

- Listens for clipboard changes and saves entries automatically
- Stores clipboard history in JSON format
- Trims old history entries when reaching a specified limit
- Saves clipboard history in `/tmp/cliphistory/clipboard_history.json`
- Efficient and lightweight

## Installation

### 1. Build the Project

Ensure you have Rust installed. If not, install Rust via [rust-lang-org](https://www.rust-lang.org/).

Clone the repository and build the binary:

```sh
  git clone <your-repo-url>
  cd cliphistory
  cargo build --release
```

### 2. Move the Binary to a System Path

To run the script globally, move the compiled binary to a directory in your `PATH`:

```sh
sudo mv target/release/cliphistory /usr/local/bin/
```

Ensure `/usr/local/bin` is in your `PATH`:

```sh
  echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc
  source ~/.zshrc
```

## Usage

### Start Listening for Clipboard Changes

```sh
  cliphistory
```

This will start monitoring clipboard changes and saving them automatically.

### View Clipboard History

```sh
  cliphistory list
```

### Clear Clipboard History

```sh
  cliphistory clear
```

## Running on Startup (macOS)

To automatically start the clipboard history manager when your computer boots:

1. Create a `plist` file:

   ```sh
   mkdir -p ~/Library/LaunchAgents
   nano ~/Library/LaunchAgents/com.user.cliphistory.plist
   ```

2. Add the following content:

   ```xml
   <?xml version="1.0" encoding="UTF-8"?>
   <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
   <plist version="1.0">
   <dict>
       <key>Label</key>
       <string>com.user.cliphistory</string>
       <key>ProgramArguments</key>
       <array>
           <string>/usr/local/bin/cliphistory</string>
       </array>
       <key>RunAtLoad</key>
       <true/>
   </dict>
   </plist>
   ```

3. Load the launch agent:

   ```sh
   launchctl load ~/Library/LaunchAgents/com.user.cliphistory.plist
   ```

This will ensure `cliphistory` runs automatically on startup.

## Configuration

Modify `HISTORY_SIZE` in `src/clip_history.rs` to change the maximum number of stored entries.

```rust
const HISTORY_SIZE: u8 = 150;
```

