# Litep2p Performance Protocol

The Litep2p Performance Protocol measures upload and download times between multiple Litep2p instances.
The `/noise` and `/yamux` protocols are negotiated automatically.

## Performance Measurements

### Bandwidth

This data showcases upload and download speeds for high-throughput performance across different communication configurations using Litep2p and Libp2p.

![Litep2p vs Libp2p Speed Comparison](images/litep2p-libp2p.png)

Here are some key insights:

- litep2p setup is **~24% faster** for upload speeds, and **~26% faster** for download speeds than libp2p
  - litep2p -> litep2p is the the fasted across all cases
    - top upload bandwidth is `608.78 Mbit/s`  (512MiB), while download is `615.73 Mbit/s` (32MiB)
  - libp2p -> libp2p is aproximately ~100 Mbit/s slower
    - top upload bandwidth is `488.77 Mbit/s` (256MiB), while download is `488.18 Mbit/s` (256MiB)

| Operation  | Bytes      | Litep2p->Litep2p | Libp2p->Libp2p | Libp2p->Litep2p | Litep2p->Libp2p |
|------------|------------|------------------|----------------|-----------------|-----------------|
| Uploaded   | 16MiB | 588.77 Mbit/s | 486.59 Mbit/s | 508.14 Mbit/s | 534.24 Mbit/s |
| Uploaded   | 32MiB | 608.04 Mbit/s | 454.59 Mbit/s | 512.72 Mbit/s | 543.04 Mbit/s |
| Uploaded   | 64MiB | 596.67 Mbit/s | 476.27 Mbit/s | 512.60 Mbit/s | 539.67 Mbit/s |
| Uploaded   | 128MiB | 597.67 Mbit/s | 475.44 Mbit/s | 500.52 Mbit/s | 566.77 Mbit/s |
| Uploaded   | 256MiB | 568.63 Mbit/s | 488.77 Mbit/s | 503.91 Mbit/s | 523.64 Mbit/s |
| Uploaded   | 512MiB | 608.78 Mbit/s | 485.44 Mbit/s | 518.24 Mbit/s | 585.06 Mbit/s |
| Uploaded   | 1,0GiB | 593.76 Mbit/s | 480.22 Mbit/s | 509.59 Mbit/s | 533.81 Mbit/s |
| Downloaded | 16MiB | 601.63 Mbit/s | 480.64 Mbit/s | 543.31 Mbit/s | 500.06 Mbit/s |
| Downloaded | 32MiB | 615.73 Mbit/s | 486.40 Mbit/s | 533.18 Mbit/s | 499.05 Mbit/s |
| Downloaded | 64MiB | 609.44 Mbit/s | 485.17 Mbit/s | 557.61 Mbit/s | 505.44 Mbit/s |
| Downloaded | 128MiB | 603.49 Mbit/s | 483.54 Mbit/s | 541.33 Mbit/s | 495.37 Mbit/s |
| Downloaded | 256MiB | 600.82 Mbit/s | 488.18 Mbit/s | 564.84 Mbit/s | 497.95 Mbit/s |
| Downloaded | 512MiB | 506.49 Mbit/s | 483.55 Mbit/s | 519.57 Mbit/s | 498.65 Mbit/s |
| Downloaded | 1,0GiB | 601.71 Mbit/s | 485.35 Mbit/s | 543.15 Mbit/s | 500.86 Mbit/s |

_Check the appendix for the full bandwidth data._

### Yamux Substreams

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

### Scripted

```bash
./run_bandwidth.sh
```

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

## Appendix

### Bandwidth

| Operation  | Bytes      | Litep2p->Litep2p | Libp2p->Libp2p | Libp2p->Libp2p | Libp2p->Litep2p | Libp2p->Litep2p | Litep2p->Libp2p |
|            |            | (TCP)            | (TCP)          | (WebRTC)       | (TCP)           | (WebRTC)        | (TCP)           |
|------------|------------|------------------|----------------|----------------|-----------------|-----------------|-----------------|
| Uploaded   | 1.0KiB | 3.46 Gbit/s | 3.16 Gbit/s | 1.17 Gbit/s | 4.36 Gbit/s | 225.36 Mbit/s | 3.74 Gbit/s |
| Uploaded   | 2.0KiB | 9.39 Gbit/s | 5.23 Gbit/s | 796.18 Mbit/s | 1.85 Gbit/s | 429.06 Mbit/s | 2.54 Gbit/s |
| Uploaded   | 4.0KiB | 2.15 Gbit/s | 6.37 Gbit/s | 1.23 Gbit/s | 8.05 Gbit/s | 370.01 Mbit/s | 3.45 Gbit/s |
| Uploaded   | 8.0KiB | 7.11 Gbit/s | 5.09 Gbit/s | 1.46 Gbit/s | 5.66 Gbit/s | 515.82 Mbit/s | 5.79 Gbit/s |
| Uploaded   | 16KiB | 838.46 Mbit/s | 373.74 Mbit/s | 1.68 Gbit/s | 391.24 Mbit/s | 532.77 Mbit/s | 800.64 Mbit/s |
| Uploaded   | 32KiB | 555.92 Mbit/s | 348.37 Mbit/s | 1.69 Gbit/s | 299.48 Mbit/s | 1.20 Gbit/s | 453.10 Mbit/s |
| Uploaded   | 64KiB | 436.65 Mbit/s | 353.20 Mbit/s | 1.74 Gbit/s | 538.50 Mbit/s | 1.73 Gbit/s | 432.45 Mbit/s |
| Uploaded   | 128KiB | 357.14 Mbit/s | 388.14 Mbit/s | 3.45 Gbit/s | 341.68 Mbit/s | 681.16 Mbit/s | 414.52 Mbit/s |
| Uploaded   | 256KiB | 482.24 Mbit/s | 323.69 Mbit/s | 51.57 Mbit/s | 321.55 Mbit/s | 60.10 Mbit/s | 349.15 Mbit/s |
| Uploaded   | 512KiB | 420.43 Mbit/s | 349.45 Mbit/s | 47.77 Mbit/s | 393.10 Mbit/s | 46.43 Mbit/s | 362.95 Mbit/s |
| Uploaded   | 1.0MiB | 452.06 Mbit/s | 382.80 Mbit/s | 34.42 Mbit/s | 405.66 Mbit/s | 35.68 Mbit/s | 396.46 Mbit/s |
| Uploaded   | 2.0MiB | 476.54 Mbit/s | 468.72 Mbit/s | 33.50 Mbit/s | 454.18 Mbit/s | 33.54 Mbit/s | 484.36 Mbit/s |
| Uploaded   | 4.0MiB | 602.35 Mbit/s | 542.05 Mbit/s | 32.51 Mbit/s | 529.74 Mbit/s | 31.15 Mbit/s | 595.09 Mbit/s |
| Uploaded   | 8.0MiB | 731.84 Mbit/s | 597.95 Mbit/s | 32.05 Mbit/s | 587.44 Mbit/s | 31.37 Mbit/s | 723.80 Mbit/s |
| Uploaded   | 16MiB | 745.03 Mbit/s | 616.75 Mbit/s | 31.83 Mbit/s | 617.36 Mbit/s | 30.80 Mbit/s | 756.98 Mbit/s |
| Uploaded   | 32MiB | 753.37 Mbit/s | 631.18 Mbit/s | 31.65 Mbit/s | 644.45 Mbit/s | n/a | 778.50 Mbit/s |
| Uploaded   | 64MiB | 806.63 Mbit/s | 635.26 Mbit/s | 31.68 Mbit/s | 632.70 Mbit/s | n/a | 784.85 Mbit/s |
| Uploaded   | 128MiB | 879.59 Mbit/s | 635.71 Mbit/s | 31.03 Mbit/s | 637.97 Mbit/s | n/a | 764.72 Mbit/s |
| Uploaded   | 256MiB | 867.99 Mbit/s | 643.30 Mbit/s | 31.42 Mbit/s | 698.84 Mbit/s | n/a | 803.06 Mbit/s |
| Uploaded   | 512MiB | 871.38 Mbit/s | 652.90 Mbit/s | 31.16 Mbit/s | 620.08 Mbit/s | n/a | 808.02 Mbit/s |
| Uploaded   | 1.0GiB | 882.42 Mbit/s | 643.92 Mbit/s | 31.39 Mbit/s | 630.04 Mbit/s | n/a | 813.40 Mbit/s |
| Downloaded | 1.0KiB | 32.95 Mbit/s | 22.49 Mbit/s | 3.77 Mbit/s | 28.53 Mbit/s | 2.36 Mbit/s | 23.87 Mbit/s |
| Downloaded | 2.0KiB | 63.92 Mbit/s | 50.13 Mbit/s | 4.88 Mbit/s | 52.36 Mbit/s | 3.45 Mbit/s | 45.08 Mbit/s |
| Downloaded | 4.0KiB | 81.83 Mbit/s | 77.48 Mbit/s | 6.82 Mbit/s | 130.75 Mbit/s | 3.54 Mbit/s | 63.33 Mbit/s |
| Downloaded | 8.0KiB | 141.94 Mbit/s | 102.54 Mbit/s | 7.62 Mbit/s | 114.75 Mbit/s | 3.58 Mbit/s | 100.94 Mbit/s |
| Downloaded | 16KiB | 195.20 Mbit/s | 169.77 Mbit/s | 9.33 Mbit/s | 190.36 Mbit/s | 5.22 Mbit/s | 141.49 Mbit/s |
| Downloaded | 32KiB | 197.94 Mbit/s | 288.42 Mbit/s | 10.47 Mbit/s | 252.40 Mbit/s | 6.81 Mbit/s | 196.53 Mbit/s |
| Downloaded | 64KiB | 225.54 Mbit/s | 286.09 Mbit/s | 11.81 Mbit/s | 296.95 Mbit/s | 10.00 Mbit/s | 275.46 Mbit/s |
| Downloaded | 128KiB | 297.69 Mbit/s | 325.77 Mbit/s | 13.88 Mbit/s | 327.18 Mbit/s | 11.41 Mbit/s | 278.86 Mbit/s |
| Downloaded | 256KiB | 413.71 Mbit/s | 357.56 Mbit/s | 19.97 Mbit/s | 351.91 Mbit/s | 18.07 Mbit/s | 351.01 Mbit/s |
| Downloaded | 512KiB | 408.00 Mbit/s | 405.74 Mbit/s | 22.14 Mbit/s | 399.83 Mbit/s | 22.16 Mbit/s | 400.23 Mbit/s |
| Downloaded | 1.0MiB | 505.80 Mbit/s | 513.97 Mbit/s | 28.11 Mbit/s | 581.64 Mbit/s | n/a | 504.74 Mbit/s |
| Downloaded | 2.0MiB | 777.57 Mbit/s | 638.82 Mbit/s | 29.31 Mbit/s | 775.99 Mbit/s | n/a | 666.08 Mbit/s |
| Downloaded | 4.0MiB | 844.21 Mbit/s | 651.80 Mbit/s | 29.96 Mbit/s | 774.64 Mbit/s | n/a | 715.37 Mbit/s |
| Downloaded | 8.0MiB | 792.89 Mbit/s | 641.97 Mbit/s | 31.05 Mbit/s | 784.99 Mbit/s | n/a | 683.01 Mbit/s |
| Downloaded | 16MiB | 803.64 Mbit/s | 626.76 Mbit/s | 31.46 Mbit/s | 745.87 Mbit/s | n/a | 688.40 Mbit/s |
| Downloaded | 32MiB | 779.37 Mbit/s | 633.95 Mbit/s | 31.48 Mbit/s | 760.55 Mbit/s | n/a | 691.93 Mbit/s |
| Downloaded | 64MiB | 804.71 Mbit/s | 635.22 Mbit/s | 31.53 Mbit/s | 752.78 Mbit/s | n/a | 690.73 Mbit/s |
| Downloaded | 128MiB | 862.24 Mbit/s | 637.05 Mbit/s | 31.18 Mbit/s | 782.09 Mbit/s | n/a | 621.61 Mbit/s |
| Downloaded | 256MiB | 858.61 Mbit/s | 636.08 Mbit/s | 31.53 Mbit/s | 788.31 Mbit/s | n/a | 694.58 Mbit/s |
| Downloaded | 512MiB | 861.56 Mbit/s | 637.50 Mbit/s | 31.19 Mbit/s | 738.45 Mbit/s | n/a | 692.77 Mbit/s |
| Downloaded | 1.0GiB | 872.73 Mbit/s | 619.31 Mbit/s | 31.04 Mbit/s | 744.37 Mbit/s | n/a | 687.94 Mbit/s |
