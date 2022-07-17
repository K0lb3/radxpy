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
    raise NotImplementedError()


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
    raise NotImplementedError()

from .radxpy import encode, decode
