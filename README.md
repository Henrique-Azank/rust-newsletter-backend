# Rust Newsletter Backend

A performant and reliable backend API for managing an email newsletter service, built with Rust.

## Features

- **Subscription Management**
- **Newsletter Distribution**
- **API Endpoints**
- **Data Persistence**
- **Security**

## Technologies

- **Rust** - Primary programming language

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/henrique-azank/rust-newsletter-backend.git
   cd rust-newsletter-backend
   ```

2. Set up environment variables:

   ```bash
   cp .env.example .env
   ```

   Edit the `.env` file with your configuration.

3. Run the application:

   ```bash
   cargo run
   ```

## API Documentation

The API follows RESTful principles and returns JSON responses.

### Endpoints

- `POST /subscriptions` - Subscribe to newsletter
- `GET /subscriptions/confirm` - Confirm subscription
- `DELETE /subscriptions/{email}` - Unsubscribe
- `POST /newsletters` - Send newsletter (admin protected)
- `GET /health_check` - Service health status

For detailed API documentation, see [API_DOCS.md](API_DOCS.md).

## Configuration

Configure the application via environment variables:

- `DATABASE_URL` - PostgreSQL connection string
- `APP_PORT` - Port to run the server (default: 8000)
- `APP_HOST` - Host to bind to (default: 127.0.0.1)
- `SMTP_*` - Email sending configuration
- `SECRET_KEY` - Application secret key

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

## Deployment

The application can be deployed using:

- Docker containers
- Traditional server deployment
- Cloud platforms (AWS, GCP, Azure)

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
