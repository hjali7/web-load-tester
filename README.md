# Web Load Tester 🚀

[![Build Status](https://github.com/YOUR_USERNAME/web-load-tester/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/web-load-tester/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/YOUR_USERNAME/web-load-tester)](https://github.com/YOUR_USERNAME/web-load-tester/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)

A powerful web load testing tool written in Rust, featuring real-time metrics, interactive charts, and a modern web dashboard.

---

<!--
**✨ Showcase Area ✨**

Consider adding a screenshot or a GIF of the web dashboard in action here!
For example:
![Web Load Tester Dashboard](https://example.com/path/to/your/screenshot.png)
-->

## 🌟 Features

- Real-time metrics visualization
- Interactive charts for latency and throughput
- Modern web dashboard
- Support for HTTP/HTTPS requests
- Concurrent request handling
- CSV export functionality
- Dark/Light theme support
- Responsive design for all devices

## 🛠️ Installation

### Using Pre-built Binaries

Replace `YOUR_USERNAME` in the download links below with the actual GitHub username or organization.

#### Ubuntu
1. Download the latest `web-load-tester-ubuntu.tar.gz` from the [Releases Page](https://github.com/YOUR_USERNAME/web-load-tester/releases)
2. Extract the archive:
   ```bash
   tar -xzf web-load-tester-ubuntu.tar.gz
   ```
3. Make the binary executable:
   ```bash
   chmod +x web-load-tester
   ```
4. Run: `./web-load-tester`

#### Windows
1. Download the latest `web-load-tester-windows.zip` from the [Releases Page](https://github.com/YOUR_USERNAME/web-load-tester/releases)
2. Extract the ZIP file
3. Run `web-load-tester.exe`

### Building from Source

#### Prerequisites
- Rust (latest stable version)
- Cargo
- OpenSSL development libraries (for Linux)

#### Ubuntu / Linux
```bash
# Install OpenSSL development libraries (if not already present)
sudo apt-get update
sudo apt-get install pkg-config libssl-dev

# Clone the repository (replace YOUR_USERNAME)
git clone https://github.com/YOUR_USERNAME/web-load-tester.git
cd web-load-tester

# Build the project
cargo build --release

# The binary will be available at target/release/web-load-tester
```

#### Windows
```bash
# Clone the repository (replace YOUR_USERNAME)
git clone https://github.com/YOUR_USERNAME/web-load-tester.git
cd web-load-tester

# Build the project
cargo build --release

# The binary will be available at target/release/web-load-tester.exe
```

## ⚙️ Usage

1. Start the application:
   ```bash
   # Ubuntu/Linux
   ./web-load-tester

   # Windows
   .\web-load-tester.exe
   ```
   (Note: On Windows, if it's in your PATH, just `web-load-tester.exe` works)

2. Open your web browser and navigate to `http://localhost:8080`

3. Configure your load test:
   - Enter the target URL
   - Set the number of concurrent users
   - Set the test duration
   - Click "Start Test"

4. View real-time results in the dashboard:
   - Monitor request success/failure rates
   - Track latency metrics
   - View throughput charts
   - Export results to CSV

## 🐳 Docker Support

You can also run the application using Docker (replace `YOUR_USERNAME` if your ghcr.io path is different, though `github.actor` is often used in CI for pushes to `ghcr.io/<actor>/<repo>`). The `ci.yml` pushes to `ghcr.io/${{ github.actor }}/web-load-tester:latest`. If your GitHub username is `YOUR_USERNAME`, then the command below is correct.

```bash
docker pull ghcr.io/YOUR_USERNAME/web-load-tester:latest
docker run -p 8080:8080 ghcr.io/YOUR_USERNAME/web-load-tester:latest
```

## 📁 Project Structure

```
web-load-tester/
├── .github/workflows/  # GitHub Actions Workflows
│   ├── ci.yml          # Build, test, lint, Docker push
│   └── release.yml     # Create releases
├── src/                # Source code
│   └── main.rs
├── static/             # Static web assets (HTML, CSS, JS)
│   └── index.html
├── Cargo.toml          # Rust package manifest
├── Dockerfile          # Docker image definition
├── LICENSE             # Project License
└── README.md           # This file
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/your-amazing-feature`).
3. Commit your changes (`git commit -m 'Add some amazing feature'`).
4. Push to the branch (`git push origin feature/your-amazing-feature`).
5. Open a Pull Request.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

HajAli

## 🙏 Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [Warp](https://github.com/seanmonstar/warp)
- [Chart.js](https://www.chartjs.org/)
- [Tailwind CSS](https://tailwindcss.com/)
---

*This README was enhanced with the help of an AI assistant.*
