# radxpy

"An ADX encoder/decoder written in Rust" extended to Python.

This module makes the [radx](https://github.com/Isaac-Lozano/radx) package available in Python.

## Installation

The module is made available made on pypi via prebuild wheels,
saving you the trouble of installing rust and building it from source.

### pipy - TODO

```cmd
pip install radxpy
```

### from source

1. install [Rust](https://www.rust-lang.org/tools/install)
2. ``pip install setuptools-rust``
3. ``python setup.py install``


## Usage

```py
from radxpy import encode, decode
```

### function signatures
```py
def encode(wav_data: bytes, start: int, end:int, no_loop: bool, ahx: bool) -> bytes:
    """
    Encodes a WAV file into an ADX file.
    Parameters
    ----------
    wav_data : bytes
        The WAV data to encode.
    start : int
        The sample to start encoding at.
        Should be 0 or greater.
    end : int
        The sample to end encoding at.
        If set to 0, the end of the WAV data will be used.
    no_loop : bool
        Whether or not to loop the song.
    ahx : bool
        Whether or not to use the ahx encoding.
    """
    ...


def decode(adx_data: bytes, loops: int) -> bytes:
    """
    Decodes an ADX file.
    Parameters
    ----------
    adx_data : bytes
        The ADX data to decode.
    loops : int
        The number of times to decode the ADX data.
        If no loops are needed, set this to 0.
    """
    ...
```

## TODO

- tests
- return PyError instead of panicing
