# Image Compressor

A fast, cross-platform desktop application for batch image compression built with Tauri v2, Rust, and Vanilla JavaScript.

## Features

- **Batch Processing**: Compress multiple images simultaneously with parallel processing
- **Multiple Quality Presets**: Choose between High (90%), Medium (75%), or Low (60%) compression
- **Format Support**: Convert images to JPEG or PNG formats
- **Smart Resizing**: Optional width-based resizing while maintaining aspect ratio
- **Fast Performance**: Leverages Rust's performance with Rayon for parallel processing
- **Cross-Platform**: Works on macOS, Windows, and Linux
- **Clean UI**: Simple, intuitive interface with real-time progress feedback

## Prerequisites

Before you begin, ensure you have the following installed on your system:

### Required Software

1. **Node.js** (v18 or higher)
   - Download from [nodejs.org](https://nodejs.org/)
   - Verify installation: `node --version`

2. **Rust** (latest stable)
   - Install via [rustup](https://rustup.rs/):

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - Verify installation: `rustc --version`

3. **System Dependencies** (platform-specific)

   **macOS:**

   ```bash
   xcode-select --install
   ```

   **Linux (Debian/Ubuntu):**

   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.1-dev \
     build-essential \
     curl \
     wget \
     file \
     libssl-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

   **Windows:**
   - Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

## Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/yourusername/image-compressor.git
   cd image-compressor
   ```

2. **Install Node.js dependencies**

   ```bash
   npm install
   ```

3. **Install Rust dependencies** (automatic on first run)
   - Cargo will automatically download and compile Rust dependencies when you first run the app

## Running the Application

### Development Mode

To run the application in development mode with hot-reload:

```bash
npm run tauri dev
```

This will:

- Start the Tauri development server
- Open the application window
- Enable hot-reload for frontend changes
- Show console logs for debugging

### Building for Production

To create an optimized production build:

```bash
npm run tauri build
```

The compiled application will be available in:

- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/deb/` or `appimage/`

## Usage

1. **Launch the application**
2. **Click the selection zone** to choose images (supports: JPG, JPEG, PNG, GIF, BMP)
3. **Configure settings**:
   - Quality: High, Medium, or Low
   - Output Format: JPEG or PNG
   - Maximum Width: Optional (leave empty to keep original dimensions)
4. **Choose output folder** where compressed images will be saved
5. **Click "Process Images"** to start compression
6. **View results** showing total, successful, and failed conversions
7. **Click "Clear"** to reset and process new images

## How Quality Settings Work

- **Quality Percentage** (High 90% / Medium 75% / Low 60%):
  - **JPEG**: Applies lossy compression (lower = smaller file, reduced visual quality)
    - Note: Converting PNG → JPEG will reduce quality regardless of setting
    - Quality setting does NOT change image dimensions
  - **PNG**: Lossless compression (maintains 100% quality from original)
    - Note: Converting JPEG → PNG won't recover lost quality
    - To reduce PNG file size, use Maximum Width to decrease dimensions

- **Maximum Width**:
  - Optional field to reduce image dimensions while maintaining aspect ratio
  - Leave empty to keep original dimensions

**Tip**: For maximum size reduction, use Low (60%) quality with JPEG format.

## Project Structure

```
image-compressor/
├── src/                          # Frontend source files
│   ├── index.html               # Main HTML file
│   ├── main.js                  # JavaScript logic
│   └── styles.css               # Application styles
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── main.rs             # Tauri entry point
│   │   ├── lib.rs              # Library exports
│   │   ├── commands.rs         # Tauri commands
│   │   ├── image_processor.rs  # Image processing logic
│   │   └── models.rs           # Data models
│   ├── capabilities/
│   │   └── default.json        # Tauri permissions
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri configuration
├── package.json                 # Node.js dependencies
└── README.md                    # This file
```

## Technology Stack

- **Frontend**: Vanilla JavaScript, HTML5, CSS3
- **Backend**: Rust
- **Framework**: Tauri v2
- **Image Processing**: `image` crate (v0.25.9)
- **EXIF Handling**: `kamadak-exif` crate (v0.5)
- **Parallel Processing**: `rayon` crate (v1.11.0)
- **Serialization**: `serde` + `serde_json`

## Troubleshooting

### Common Issues

**Issue**: `npm run tauri dev` fails with "command not found"

- **Solution**: Ensure `@tauri-apps/cli` is installed: `npm install`

**Issue**: Rust compilation errors

- **Solution**: Update Rust to the latest version: `rustup update`

**Issue**: Images fail to process

- **Solution**: Ensure output directory has write permissions

**Issue**: Application won't start on Linux

- **Solution**: Install missing system dependencies (see Prerequisites)

### Getting Help

If you encounter issues:

1. Check the [Tauri documentation](https://tauri.app/)
2. Review the console logs in development mode
3. Open an issue on GitHub with error details

## Performance

- **Parallel Processing**: Utilizes all available CPU cores via Rayon
- **Memory Efficient**: Processes images in batches without loading all into memory
- **Fast Compression**: Rust-based processing is significantly faster than JavaScript alternatives

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- Extensions:
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Roadmap

- [ ] Drag & drop file selection
- [ ] EXIF metadata privacy controls
- [ ] Batch rename options
- [ ] Image preview before/after
- [ ] Custom compression quality slider
