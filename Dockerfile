# Use an official OpenJDK runtime as the base image
FROM rust:1.67 AS builder

WORKDIR /distrust
COPY . .
RUN cargo build --release

FROM openjdk:11-jdk-slim

# Set environment variables
ENV MAELSTROM_VERSION=0.2.3
ENV MAELSTROM_DIR=/opt/maelstrom

# Install dependencies: curl for downloading files, Graphviz, and gnuplot
RUN apt-get update && apt-get install -y --no-install-recommends \
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
