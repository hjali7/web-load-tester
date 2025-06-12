# Web Load Tester

A modern, real-time web load testing tool built with Rust and a beautiful web dashboard. This tool helps you test the performance and reliability of your web applications under load.

## Features

- 🚀 **Real-time Metrics**: Monitor test results in real-time with a modern web dashboard
- 📊 **Interactive Charts**: Visualize latency and request patterns with dynamic charts
- 🌓 **Dark/Light Mode**: Toggle between dark and light themes for comfortable viewing
- 📈 **Comprehensive Metrics**:
  - Total Requests
  - Success Rate
  - Average Latency
  - Requests per Second
  - Min/Max Latency
- 📥 **Data Export**: Export test results to CSV for further analysis
- 🔄 **Concurrent Testing**: Simulate multiple users with configurable concurrency
- ⏱️ **Ramp-up Support**: Gradually increase load with configurable ramp-up time
- 🎨 **Responsive Design**: Works seamlessly on both desktop and mobile devices

## Prerequisites

- Rust 1.75 or higher
- OpenSSL development libraries
- A modern web browser

### Installing Dependencies

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install pkg-config libssl-dev
```

#### Windows
1. Install OpenSSL from [OpenSSL for Windows](https://slproweb.com/products/Win32OpenSSL.html)
2. Add OpenSSL to your system PATH

#### macOS
```bash
brew install openssl
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/web-load-tester.git
cd web-load-tester
```

2. Build the project:
```bash
cargo build --release
```

## Usage

1. Start the server:
```bash
cargo run --release
```

2. Open your browser and navigate to:
```
http://localhost:3000
```

3. Configure your test:
   - Enter the target URL
   - Set the number of concurrent users
   - Define test duration
   - Set ramp-up time

4. Click "Start Test" to begin the load test

5. Monitor real-time results in the dashboard

6. Export results to CSV when the test is complete

## Docker Support

Build and run using Docker:

```bash
# Build the image
docker build -t web-load-tester .

# Run the container
docker run -p 3000:3000 web-load-tester
```

## Development

### Project Structure
```
web-load-tester/
├── src/
│   └── main.rs          # Main application code
├── static/
│   └── index.html       # Web dashboard
├── Cargo.toml           # Rust dependencies
├── Dockerfile          # Docker configuration
└── README.md           # This file
```

### Building from Source
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
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

Your Name - [Your GitHub Profile](https://github.com/yourusername)

## Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [Warp](https://github.com/seanmonstar/warp)
- [Chart.js](https://www.chartjs.org/)
- [Tailwind CSS](https://tailwindcss.com/) 