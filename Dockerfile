# ---- Build Stage ----
FROM rust:latest as builder
WORKDIR /app
    
# 소스 복사 및 빌드
COPY . .
RUN cargo build --release
    
# ---- Runtime Stage ----
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/rorscha_server /app/rorscha_server
    
# 진입점 설정
ENTRYPOINT ["/app/rorscha_server"]