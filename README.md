# radxpy

"An ADX encoder/decoder written in Rust" extended to Python.

This module makes the [radx](https://github.com/Isaac-Lozano/radx) package available in Python.

## Installation

The module is made available made on pypi via prebuild wheels,
saving you the trouble of installing rust and building it from source.

### pipy

```cmd
pip install radxpy
```

### from source

1. install [Rust](https://www.rust-lang.org/tools/install)
2. ``pip install setuptools-rust``
3. ``python setup.py install``

## TODO

- tests
- return PyError instead of panicing
