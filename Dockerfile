####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev binutils upx
RUN update-ca-certificates

# Create appuser
ENV USER=rust
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /rust-percentage-change-calculator

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN strip target/x86_64-unknown-linux-musl/release/rust-percentage-change-calculator

RUN upx --best --lzma target/x86_64-unknown-linux-musl/release/rust-percentage-change-calculator

####################################################################################################
## Final Image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rust-percentage-change-calculator

# Copy our build
COPY --from=builder /rust-percentage-change-calculator/target/x86_64-unknown-linux-musl/release/rust-percentage-change-calculator ./

# Use an unprivileged user.
USER rust:rust

ENTRYPOINT ["./rust-percentage-change-calculator"]
