# Rust Server Template

A production-ready Rust web server template using Actix-web and Diesel ORM. Features include authentication, organization management, health monitoring, and comprehensive testing infrastructure.

## Features

- 🚀 Actix-web for high-performance HTTP serving
- 🗄️ Diesel ORM with PostgreSQL
- 🔐 JWT-based authentication
- 📝 OpenAPI/Swagger documentation
- 🐳 Docker and Docker Compose setup
- 🔄 Hot reloading for development
- ✅ Comprehensive test suite (unit, integration, performance)
- 📊 Health monitoring and metrics
- 🎯 Rate limiting and security headers
- 📦 Repository pattern for database operations

## Prerequisites

- Docker
- Docker Compose
- Git

## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/jonahduckworth/create_rust_server.git
cd create_rust_server
```

2. Create a `.env` file in the root directory:

```env
DATABASE_URL=postgres://postgres:postgres@db:5432/postgres
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=postgres
POSTGRES_PORT_EXTERNAL=5432
API_PORT=8080
JWT_SECRET=your-secret-key
ENVIRONMENT=development
RUST_LOG=debug
```

3. Build and start the containers:

```bash
# Stop any existing containers and remove volumes
docker-compose down -v

# Build and start fresh
docker-compose up --build
```

The API will be available at `http://localhost:8080`

## API Documentation

Access the Swagger UI documentation at: `http://localhost:8080/swagger-ui/`

### Available Endpoints

#### Health Checks

```
GET /v1/health
GET /v1/health/live
GET /v1/health/ready
```

#### Authentication

```
POST /v1/auth/login
{
    "email": "user@example.com",
    "password": "password123"
}

POST /v1/auth/register
{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john@example.com",
    "phone_number": "1234567890",
    "password": "password123",
    "org_id": "uuid-string"
}

POST /v1/auth/refresh
{
    "refresh_token": "token-string"
}
```

#### Organizations

```
GET    /v1/organizations
GET    /v1/organizations/{id}

POST   /v1/organizations
{
    "name": "Organization Name"
}

PUT    /v1/organizations/{id}
{
    "name": "Updated Organization Name"
}

DELETE /v1/organizations/{id}
```

## Development

The project uses Docker for development with hot-reloading enabled. Any changes to Rust files will automatically trigger a rebuild.

### Running Tests

```bash
# Run all tests
docker-compose exec app cargo test

# Run specific test suite
docker-compose exec app cargo test integration
docker-compose exec app cargo test unit
docker-compose exec app cargo test performance
```

### Project Structure

```
src/
├── api/           # HTTP API implementation
├── db/            # Database models and repositories
├── domain/        # Business logic
├── error/         # Error handling
├── tests/         # Test suites
└── utils/         # Shared utilities
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
