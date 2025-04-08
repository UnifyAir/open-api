# Open-Api
A Rust-based implementation of the 5G Core network r17 OpenAPI specifications.

## Overview
This project provides OpenAPI implementations for SBI calls, for primary NFs.

## Project Structure
- `oasbi/` - OpenAPI models for nfs
- `openapi-nfs/` - NFs client deserialization and axum server implementation
- `macros/` - Custom macros for the project
- `scripts/` - Utility scripts
- `retry-after/` - Retry after authorization header

## Todo:
- [ ] Multipart Support in Deserialization
- [ ] Multipart Support for Axum Server

## Requirements
- Rust (see `rust-toolchain.toml` for version)
- Cargo

## Building
```bash
cargo build
```