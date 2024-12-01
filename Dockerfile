# Use cargo-chef to cache Docker layers on prereqs
FROM rust:1.74 AS chef
WORKDIR /distrust
RUN cargo install --version 0.1.68 cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /distrust/recipe.json recipe.json
# Build dependencies (cached)
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM openjdk:24-slim

# Set environment variables
ENV MAELSTROM_VERSION=0.2.3
ENV MAELSTROM_DIR=/opt/maelstrom

# Install dependencies: curl for downloading files, Graphviz, and gnuplot
RUN apt-get update && apt-get install -y --no-install-recommends \
    # RUN apk add --no-cache \
    graphviz \
    gnuplot \
    bzip2 \
    curl \
    git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create a directory for Maelstrom
RUN mkdir -p $MAELSTROM_DIR

# Download and extract Maelstrom
WORKDIR $MAELSTROM_DIR
RUN curl -L https://github.com/jepsen-io/maelstrom/releases/download/v$MAELSTROM_VERSION/maelstrom.tar.bz2 | tar xvj
ENV PATH="$PATH:$MAELSTROM_DIR/maelstrom"

COPY --from=builder /distrust/target/release/ ${MAELSTROM_DIR}/bin


# Expose port for Maelstrom's web server
EXPOSE 8080

# Default command to start the Maelstrom web server
CMD ["maelstrom", "serve"]
