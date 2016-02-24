Serde YAML Serialization Library
================================

[![Build Status](https://api.travis-ci.org/dtolnay/serde-yaml.svg?branch=master)](https://travis-ci.org/dtolnay/serde-yaml)

This crate is a Rust library for using the [Serde](https://github.com/serde-rs/serde)
serialization framework with data in [YAML](http://yaml.org) file format. This
library does not reimplement a YAML parser; it uses [yaml-rust](https://github.com/chyh1990/yaml-rust)
which is a pure Rust YAML 1.2 implementation.

Installation
============

[TODO not published yet] This crate works with Cargo and can be found on
[crates.io](https://crates.io/crates/serde_yaml) with a `Cargo.toml` like:

```toml
[dependencies]
serde = "*"
serde_yaml = "*"
```

Using Serde YAML
================

```rust
extern crate serde;
extern crate serde_yaml;

use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("x".to_string(), 1.0);
    map.insert("y".to_string(), 2.0);

    let s = serde_yaml::to_string(&map).unwrap();
    assert_eq!(s, "---\n\"x\": 1\n\"y\": 2");

    let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&s).unwrap();
    assert_eq!(map, deserialized_map);
}
```

It can also be used with Serde's automatic serialization library,
`serde_macros`. First add this to `Cargo.toml`:

```toml
[dependencies]
serde = "*"
serde_macros = "*"
serde_yaml = "*"
```

Then use:

```rust
#![feature(plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_yaml;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };

    let s = serde_yaml::to_string(&point).unwrap();
    assert_eq!(s, "---\n\"x\": 1\n\"y\": 2");

    let deserialized_point: Point = serde_yaml::from_str(&s).unwrap();
    assert_eq!(point, deserialized_point);
}
```
