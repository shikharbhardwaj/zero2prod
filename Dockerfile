# This Dockerfile is split into 3 stages to cache dependencies and only
# recompile the app in most cases.
#
# Uses cargo-chef (ref: https://github.com/LukeMathWalker/cargo-chef)
# to compute a recipe.json for compiling dependencies separately from the app.
FROM lukemathwalker/cargo-chef:latest-rust-1.69.0 as chef

WORKDIR /app
RUN apt update && apt install lld clang -y

# This stage computes the recipe.json
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# This stage compiles the app. If recipe.json from the previous stage remains
# unchanged, the cargo chef stage should be skipped, leading to faster container
# build times.
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

# This stage runs the app.
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# Install OpenSSL, as it is dynamically linked by Rust dependencies.
# Install ca-certs to install common certificate authorities certs to allow
# verifying TLS certs for HTTPS connections.
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production

CMD ["./zero2prod"]