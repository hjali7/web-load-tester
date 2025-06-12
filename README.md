# Web Load Tester

A powerful web load testing tool written in Rust, featuring real-time metrics, interactive charts, and a modern web dashboard.

## Features

- Real-time metrics visualization
- Interactive charts for latency and throughput
- Modern web dashboard
- Support for HTTP/HTTPS requests
- Concurrent request handling
- CSV export functionality
- Dark/Light theme support
- Responsive design for all devices

## Installation

### Using Pre-built Binaries

#### Ubuntu
1. Download the latest release from the [Releases](https://github.com/YOUR_USERNAME/web-load-tester/releases) page
2. Extract the archive:
```bash
tar -xzf web-load-tester-ubuntu.tar.gz
```
3. Make the binary executable:
```bash
chmod +x web-load-tester
```

#### Windows
1. Download the latest release from the [Releases](https://github.com/YOUR_USERNAME/web-load-tester/releases) page
2. Extract the ZIP file
3. Run `web-load-tester.exe`

### Building from Source

#### Prerequisites
- Rust (latest stable version)
- Cargo
- OpenSSL development libraries

#### Ubuntu
```bash
# Install OpenSSL development libraries
sudo apt-get update
sudo apt-get install pkg-config libssl-dev

# Clone the repository
git clone https://github.com/YOUR_USERNAME/web-load-tester.git
cd web-load-tester

# Build the project
cargo build --release

# The binary will be available at target/release/web-load-tester
```

#### Windows
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/web-load-tester.git
cd web-load-tester

# Build the project
cargo build --release

# The binary will be available at target/release/web-load-tester.exe
```

## Usage

1. Start the application:
```bash
# Ubuntu
./web-load-tester

# Windows
web-load-tester.exe
```

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

## Docker Support

You can also run the application using Docker:

```bash
docker pull ghcr.io/YOUR_USERNAME/web-load-tester:latest
docker run -p 8080:8080 ghcr.io/YOUR_USERNAME/web-load-tester:latest
```

## Project Structure

```
web-load-tester/
├── src/
│   └── main.rs
├── static/
│   └── index.html
├── Cargo.toml
├── Dockerfile
├── LICENSE
└── README.md
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

HajAli

## Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [Warp](https://github.com/seanmonstar/warp)
- [Chart.js](https://www.chartjs.org/)
- [Tailwind CSS](https://tailwindcss.com/) 