# aligned-file

A cross-platform Rust library for storing data in an aligned file. Everything in the file is aligned, so it works with direct and asynchronous I/O (`io_uring` and `O_DIRECT` on Linux, and native equivalents on other systems). Freed space is tracked and reused, so the file stays compact.

> **Status: under active development.** The format and API are unstable and will change. Not ready for production use. A stable `extern "C"` API and clients for other languages are planned.

## Contents

- [Why](#why)
- [Highlights](#highlights)
- [Platforms](#platforms)
- [How it works](#how-it-works)
- [Features](#features)
- [Repacker](#repacker)
- [C API and language clients](#c-api-and-language-clients)
- [Use cases](#use-cases)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Why

Direct I/O skips the OS page cache and talks to the disk directly. For that, every buffer address, file offset and read or write length must be aligned to the disk block size, otherwise the operation fails.

`aligned-file` takes care of this. Data is stored in aligned blocks, freed blocks are reused for new data, and your code works with plain byte slices.

## Highlights

- Alignment from 2 bytes to 64 KiB, stored in the file header.
- Compact storage: free blocks are tracked and reused before the file grows.
- Reader, writer and allocator services.
- Batched asynchronous reads and writes.
- Separate I/O backend for each operating system.
- Repacker that changes alignment and removes gaps.
- `extern "C"` API and clients for other languages.

## Platforms

The file format is the same everywhere. Only the I/O layer differs per platform.

| Platform | Direct I/O | Async I/O | Status |
|----------|------------|-----------|--------|
| Linux | `O_DIRECT` | `io_uring` | primary target |
| Windows | `FILE_FLAG_NO_BUFFERING` | IOCP | planned |
| macOS | `F_NOCACHE` | thread pool | planned |
| Other | buffered I/O | thread pool | fallback |

A file created on one platform can be opened on another. If the target disk needs a different alignment, use the repacker.

## How it works

The data file consists of a header and aligned blocks of data. The header holds a magic number, the format version and the alignment. Information about free blocks is kept in a separate `<name>.metadata` file next to the data file.

When you write data, the library looks for a free block that fits. If there is none, the file grows. When you delete data, its block becomes free and is reused by later writes.

## Features

This is not the full list. The library is in development, and more functionality will be added over time.

**Allocator.** Chooses where data is written. Reuses free blocks, splits large ones, merges neighbours and grows the file only when needed. Reports statistics such as free space and fragmentation.

**Writer.** Writes data and returns a handle to it. Supports updating data in place, moving it when it no longer fits, batch writes and explicit flush to disk.

**Reader.** Reads data by handle. Supports reading part of a record, batch reads and a sequential scan over all stored records.

**Aligned buffers.** Buffers with the required memory alignment, plus a buffer pool for reuse. Callers can pass ordinary byte slices and the library handles the rest.

**Metadata.** Saves and loads the list of free blocks, so space reuse survives restarts. The list is checked on load, and a damaged metadata file never makes the data unreadable.

## Repacker

A tool for maintaining existing files.

- Changes the alignment of a file, for example when moving it to a disk with a different block size.
- Removes gaps and shrinks the file.
- Provides a mapping from old to new positions of the data.
- Works safely: the original file is replaced only after the new one is complete.
- Has a dry-run mode that shows how much space can be reclaimed.

## C API and language clients

The library provides a stable `extern "C"` API. All clients are thin wrappers over it, so every language gets the same behavior and the same file format.

The API is delivered as a shared library (`.so`, `.dll`, `.dylib`), a static library and a generated C header.

Because any language with a C FFI can call the C API, the library will be usable from most programming languages. Official clients for popular languages will be built on top of it as the core stabilizes.

## Use cases

- **Databases and storage engines** that manage their own cache.
- **Key-value stores** with reuse of space freed by deletes and overwrites.
- **Disk caches** for large objects without double caching in the OS.
- **Queues and event stores** where blocks are reused after consumption.
- **Fast NVMe services** that need many parallel asynchronous requests.
- **Applications in other languages** that need fast local storage without writing native code.
- **Systems in several languages** that share one file format and one implementation.

Less suitable for very small files, rare access, or workloads that rely on the OS page cache.

## Roadmap

- [x] Alignment type and rounding helpers
- [x] File header, metadata and free-block structures
- [ ] Error handling across the public API
- [ ] Aligned buffers and buffer pool
- [ ] Allocator with free-block reuse
- [ ] Writer and reader services
- [ ] Linux backend: `io_uring` and `O_DIRECT`
- [ ] Fallback backend
- [ ] Windows backend
- [ ] macOS backend
- [ ] Metadata persistence
- [ ] Repacker
- [ ] `extern "C"` API
- [ ] Clients for other languages
- [ ] CI on Linux, Windows and macOS

## Contributing[LICENSE](LICENSE)

The project is at an early stage and the design is still moving. Issues with ideas and feedback are welcome. For larger changes, please open an issue before sending a pull request.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project is licensed under the same terms, without any additional terms or conditions.