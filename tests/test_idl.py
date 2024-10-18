from anchorpy_idl import (
    Idl,
    IdlField,
    IdlAccount,
    IdlTypeSimple,
    IdlTypeDef,
    IdlTypeDefAlias,
    IdlErrorCode,
    IdlTypeDefStruct,
)

from pathlib import Path


def test_idl_type_simple_hash() -> None:
    assert isinstance(hash(IdlTypeSimple.Bool), int)


def test_idls() -> None:
    idls = []
    for path in Path("tests/idls/").iterdir():
        raw = path.read_text()
        idl = Idl.from_json(raw)
        idls.append(idl)
    assert idls
