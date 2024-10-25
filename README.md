# Optimizer API

This project is a Rust-based API for the Optimizer application. It uses Actix-web as the web framework and Diesel for ORM.

## Prerequisites

- Docker
- Docker Compose

## Getting Started

1. Clone the repository:

   ```
   git clone https://github.com/jd-builds/forestry_optimizer_backend.git
   cd forestry_optimizer_backend
   ```

2. Create a `.env` file in the root directory and set the environment variables.

3. Build and start the Docker containers:

   ```
   docker-compose up --build
   ```

   This command will build the Docker image and start the containers for the API and the PostgreSQL database.

4. The API will be available at `http://localhost:8080`.

5. To stop the containers, use:
   ```
   docker-compose down
   ```

## API Documentation

Once the server is running, you can access the Swagger UI documentation at:

`http://localhost:8080/swagger-ui/`

This provides an interactive interface to explore and test the API endpoints.

## Development

The project uses Docker for development. The `Dockerfile.dev` is configured for hot-reloading, so any changes you make to the Rust files will automatically trigger a rebuild of the project.

To run the development server:

```
docker-compose up
```
