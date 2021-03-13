# Builder stage
FROM rust:1.50 as builder

WORKDIR app
COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
from debian:buster-slim AS runtime
WORKDIR app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/main hilow

ENTRYPOINT ["./hilow"]
