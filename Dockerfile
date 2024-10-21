# Use the official Rust image as the base image
FROM rust:1.78 as builder

# Install netcat
RUN apt-get update && apt-get install -y netcat-openbsd

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml file
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy source code
RUN rm -rf src

# Copy the entire project
COPY . .

# Build the application
RUN cargo build --release

# Create a shell script to wait for the database
RUN echo '#!/bin/sh\n\
while ! nc -z db 5432; do\n\
  echo "Waiting for database..."\n\
  sleep 1\n\
done\n\
echo "Database is up - executing command"\n\
exec "$@"' > /usr/local/bin/wait-for-db.sh \
&& chmod +x /usr/local/bin/wait-for-db.sh

# Set the startup command
CMD ["/usr/local/bin/wait-for-db.sh", "cargo", "run", "--release"]

ENV DATABASE_URL=postgres://forestry:optimizer@localhost/forestryoptimizer

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Create entrypoint script
RUN echo '#!/bin/sh\n\
echo "Current directory: $(pwd)"\n\
echo "Listing migrations directory:"\n\
ls -la /usr/src/app/migrations\n\
echo "Running migrations..."\n\
diesel migration run\n\
echo "Migrations completed."\n\
exec "$@"' > /usr/local/bin/docker-entrypoint.sh \
&& chmod +x /usr/local/bin/docker-entrypoint.sh

# Copy migrations folder
COPY migrations /usr/src/app/migrations

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

RUN ls -la /usr/src/app/migrations
