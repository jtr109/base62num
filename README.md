# base62num

[![Crates.io](https://img.shields.io/crates/d/base62num)](https://crates.io/crates/base62num)
[![License](https://img.shields.io/crates/l/base62num)](https://github.com/jtr109/base62num/blob/master/LICENSE)
[![Coverage Status](https://codecov.io/gh/jtr109/base62num/branch/master/graph/badge.svg)](https://codecov.io/gh/jtr109/base62num)


A convertor between numbers and strings in [Base62](https://en.wikipedia.org/wiki/Base62).

## Alphanumeric

This library using [the Base62 index table on Wikipedia](https://en.wikipedia.org/wiki/Base62#Base62_table), which is in the order of "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789" that familiar with [Base64](https://en.wikipedia.org/wiki/Base64#Base64_table).

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
