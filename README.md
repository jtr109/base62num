# base62num

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT)

A convertor between numbers and strings in [Base62](https://en.wikipedia.org/wiki/Base62).

## Usages

Using the crate as a dependency.

```toml
[dependencies]
base62num = "0.1"
```

### Encoding A Number into A String in Base62

```rust
use base62num::encode;

assert_eq!(encode(123), "B9");
```

### Decoding A String in Base62 into A Number

```rust
use base62num::decode;

assert_eq!(decode("B9"), Some(123));
```

## License

`base62num` is under [the MIT license](./LICENSE).
