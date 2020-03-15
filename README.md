# iid

Dead simple numeric IDs

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Atlas-Rhythm/iid/Tests?style=flat-square)](https://github.com/Atlas-Rhythm/iid/actions?query=workflow%3ATests)

`iid` lets you generate unique 52 bits IDs, with the whole API consisting of a single `gen` function.
IDs never take more than 52 bits, so they are always valid numbers in JavaScript, which makes this library perfect for IDs used in web APIs.

## Structure

```
+---------+--------------------------+------------+
| Padding | Timestamp                | Serial     |
+---------+--------------------------+------------+

Padding --- [12 bits] - 0 padding to fill an unsigned 64 bits integer
Timestamp - [36 bits] - 36 least significant bits of the current Unix timestamp
Serial ---- [16 bits] - Automatically increasing number starting at 0 for each timestamp
```

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).
