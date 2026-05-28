<div align="center">
  <img src="assets/icon.png" alt="kayleeping Icon" width="128" height="128">
  <h1>kayleeping</h1>
  <p>
    A small, automated "drop" system that periodically fetches encrypted content, decrypts it locally with a shared password, and displays it (image/text) on the target machine. It's less like messaging in the traditional sense and more like a scheduled, one-way encrypted content delivery system.
  </p>
  <p>
    <em>Originally designed to send my girlfriend goofy pictures occasionally.</em>
  </p>
</div>

## Installation

### 1. Download the Latest Release

Download the latest `.app` release from [GitHub Releases](https://github.com/CFdefense/kayleeping/releases)

- **macOS**: Download the `.app` bundle and move it to your Applications folder
- **Linux**: Download the appropriate binary for your system

#### macOS: Remove Quarantine Attribute

After downloading, remove the quarantine attribute to prevent "damaged app" warnings:

```bash
xattr -cr /Applications/kayleeping.app
```

### 2. Setup Configuration Directory

Create the kayleeping configuration directory:

#### macOS

```bash
mkdir -p ~/Library/Application\ Support/kayleeping
```

#### Linux

```bash
mkdir -p ~/.local/share/kayleeping
```

### 3. Create Password File

Create a `.env` file in the configuration directory with your password:

#### macOS

```bash
echo "PASSWORD=your-password-here" > ~/Library/Application\ Support/kayleeping/.env
```

#### Linux

```bash
echo "PASSWORD=your-password-here" > ~/.local/share/kayleeping/.env
```

**Important**: The `.env` file must contain exactly:

```
PASSWORD=your-password-here
```

Make sure there's no `#` comment symbol at the start of the line.

## Configuration Locations

The app automatically looks for configuration in these platform-specific locations:

- **macOS**: `~/Library/Application Support/kayleeping/.env`
- **Linux**: `~/.local/share/kayleeping/.env`

## Running the App

### GUI Mode (Default)

Simply launch the app:

- **macOS**: Double-click the app in Applications or run from terminal
- **Linux**: Run the binary from terminal

```bash
./kayleeping
```

The app will:

1. Load your PASSWORD from the `.env` file
2. Fetch encrypted content from the remote GitHub repository
3. Decrypt and display the image with caption
4. Show helpful error messages if something goes wrong

### CLI Mode (Encrypt Content)

To encrypt new content for distribution:

```bash
PASSWORD='your-password' ./kayleeping /path/to/image.png "Your caption text"
```

This encrypts the image and text, writing ciphertext to `data/source/img.enc` and `data/source/txt.enc`. You can then publish these encrypted files to your GitHub repository.

## Building from Source

Requirements:

- Rust toolchain (latest stable)
- Cargo

```bash
# Clone the repository
git clone https://github.com/CFdefense/kayleeping.git
cd kayleeping

# Create .env file in project root or set PASSWORD environment variable
echo "PASSWORD=your-password" > .env

# Run in development mode
cargo run

# Build release binary
cargo build --release

# The binary will be at: target/release/kayleeping
```

### Encrypting Content for Distribution

```bash
# Encrypt an image with caption
cargo run -- /path/to/image.png "Your caption text"

# Or with PASSWORD in environment
PASSWORD='your-password' cargo run -- /path/to/image.png "Caption"
```

After encrypting, commit and push the `data/source/*.enc` files to your GitHub repository. The app fetches from the URLs defined in `src/content.rs`:

- `REMOTE_IMG_URL`: Points to `img.enc`
- `REMOTE_TEXT_URL`: Points to `txt.enc`

## Technical Stack

- **Rust** - Core language
- **iced** - Cross-platform GUI framework
- **reqwest** - HTTP client for fetching encrypted content
- **AES-GCM** - Authenticated encryption
- **PBKDF2** - Password-based key derivation
- **SHA-2** - Cryptographic hashing

## Troubleshooting

### Password Not Found Error

If you see "PASSWORD not found in environment or .env file":

1. **Verify file location**: Check that `.env` exists in the correct directory:
   - macOS: `~/Library/Application Support/kayleeping/.env`
   - Linux: `~/.local/share/kayleeping/.env`

2. **Check file contents**: The file must contain `PASSWORD=your-password` with no `#` at the start

3. **Check permissions**: Ensure the file is readable:
   ```bash
   chmod 600 ~/.local/share/kayleeping/.env  # Linux
   chmod 600 ~/Library/Application\ Support/kayleeping/.env  # macOS
   ```

### Network Errors

If content fails to fetch:

- Verify your internet connection
- Check that the GitHub repository is accessible
- Ensure encrypted content files exist at the remote URLs in `src/content.rs`

### Decryption Errors

If decryption fails:

- Verify your PASSWORD matches the one used to encrypt the content
- Check that the encrypted files are not corrupted
- Ensure you're using compatible versions (encrypt and decrypt with same app version)

### GUI Doesn't Launch

- **macOS**: If you see a security warning, go to System Preferences → Security & Privacy and allow the app
- **Linux**: Ensure the binary has execute permissions: `chmod +x kayleeping`

## How It Works

1. **Encryption**: Content creator encrypts an image and caption text using their password
2. **Distribution**: Encrypted files (`img.enc`, `txt.enc`) are published to GitHub
3. **Fetching**: The app downloads encrypted content from GitHub
4. **Decryption**: Using the shared password, the app decrypts the content locally
5. **Display**: The decrypted image and caption are displayed in a GUI window

All decryption happens locally on your machine. The password never leaves your device.

## Security Notes

- Uses AES-GCM authenticated encryption
- Password is derived using PBKDF2 with SHA-256
- Encrypted content is stored on GitHub (public or private repository)
- Decryption only occurs on the local machine with the correct password

## License

See repository for license details.
