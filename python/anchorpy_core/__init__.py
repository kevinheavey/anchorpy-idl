from anchorpy_idl.anchorpy_idl import (  # type: ignore
    idl,
    __version__ as _version_untyped,
)

__all__ = ["idl"]

__version__: str = _version_untyped
