# Litep2p Performance Protocol

The Litep2p Performance Protocol measures upload and download times between multiple Litep2p instances.
The `/noise` and `/yamux` protocols are negotiated automatically.

## Performance Measurements

| Substreams | Average Time to Open Substreams Litep2p |
|------------|--------------------------------|
| 1        | 528.738µs |
| 32        | 2.418918ms |
| 64        | 3.993045ms |
| 128        | 8.424956ms |
| 256        | 11.428443ms |

## Protocol Specification

The protocol identifier is `/litep2p-perf/1.0.0`, and it operates in two modes, client and server.

### Client Mode

1. Connects to the server.
2. Sends a u64 big-endian value indicating the number of bytes to upload.
3. Uploads the specified number of bytes.
4. Sends a u64 big-endian value indicating the number of bytes to download.
5. Downloads the specified number of bytes.

### Server Mode

1. Listens for client connections.
2. Reads a u64 big-endian value specifying the expected upload size.
3. Receives the specified number of bytes.
4. Reads a u64 big-endian value specifying the expected download size.
5. Sends the specified number of bytes to the client.


## Network Bandwidth

### Server

```bash
RUST_LOG=info cargo run -- server --listen-address "/ip6/::/tcp/33333" --node-key "secret"
```

### Client

```bash
RUST_LOG=info cargo run -- client --server-address "/ip6/::1/tcp/33333/p2p/12D3KooWBpZHDZu7YSbvPaPXKhkRNJvR7MkTJMQQAVBKx9mCqz3q" --upload-bytes 1024 --download-bytes 0
```

## Time to Open Substreams

### Server

```bash
RUST_LOG=info cargo run -- server --listen-address "/ip6/::/tcp/33333" --node-key "secret"
```

### Client

```bash
RUST_LOG=info cargo run -- client-substream --server-address "/ip6/::1/tcp/33333/p2p/12D3KooWBpZHDZu7YSbvPaPXKhkRNJvR7MkTJMQQAVBKx9mCqz3q" --substreams 32
```

### Scripted

```bash
cd litep2p
./run_substreams.sh
```
