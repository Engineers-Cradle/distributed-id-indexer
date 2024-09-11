## Distributed ID Indexing System

### Introduction

This project is a distributed ID indexing system that allows for the storage and retrieval of IDs. The system is composed of a set of nodes that are responsible for storing and indexing IDs using Redis and Redis Pub/Sub. The system is designed to be fault-tolerant and scalable.

### 🦄 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### 🛠️ Configuration

The system can be configured using the following environment variables:

```
REDIS_URL=
WEB_SERVER_PORT=
NUM_WORKERS=
LOG_LEVEL=
```

### 🚀 Usage

```bash
$ cargo run
```

### 📝 License