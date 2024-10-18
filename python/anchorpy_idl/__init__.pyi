from typing import Union, Sequence, List, Optional, Tuple
from jsonalias import Json

class IdlTypeSimple:
    Bool: "IdlTypeSimple"
    U8: "IdlTypeSimple"
    I8: "IdlTypeSimple"
    U16: "IdlTypeSimple"
    I16: "IdlTypeSimple"
    U32: "IdlTypeSimple"
    I32: "IdlTypeSimple"
    F32: "IdlTypeSimple"
    U64: "IdlTypeSimple"
    I64: "IdlTypeSimple"
    F64: "IdlTypeSimple"
    U128: "IdlTypeSimple"
    I128: "IdlTypeSimple"
    U256: "IdlTypeSimple"
    I256: "IdlTypeSimple"
    Bytes: "IdlTypeSimple"
    String: "IdlTypeSimple"
    Pubkey: "IdlTypeSimple"
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __hash__(self) -> int: ...

class IdlTypeDefined:
    def __init__(self, defined: str) -> None: ...
    @property
    def defined(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeDefined", op: int) -> bool: ...
    def __hash__(self) -> int: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeDefined": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeDefined": ...

class IdlTypeOption:
    def __init__(self, option: IdlType) -> None: ...
    @property
    def option(self) -> IdlType: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeOption", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeOption": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeOption": ...

class IdlTypeVec:
    def __init__(self, vec: IdlType) -> None: ...
    @property
    def vec(self) -> IdlType: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeVec", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeVec": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeVec": ...

class IdlTypeArray:
    def __init__(self, array: Tuple[IdlType, int]) -> None: ...
    @property
    def array(self) -> Tuple[IdlType, int]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeArray", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeArray": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeArray": ...

class IdlTypeGenericLenArray:
    def __init__(self, generic_len_array: Tuple[IdlType, str]) -> None: ...
    @property
    def generic_len_array(self) -> Tuple[IdlType, str]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeGenericLenArray", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeGenericLenArray": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeGenericLenArray": ...

class IdlTypeGeneric:
    def __init__(self, generic: str) -> None: ...
    @property
    def generic(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeGeneric", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeGeneric": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeGeneric": ...

IdlDefinedTypeArg = Union[IdlType, IdlTypeGeneric, str]

class IdlTypeDefinedWithTypeArgs:
    def __init__(self, name: str, args: Sequence[IdlDefinedTypeArg]) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def args(self) -> IdlDefinedTypeArg: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeDefinedWithTypeArgs", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeDefinedWithTypeArgs": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeDefinedWithTypeArgs": ...

class IdlConst:
    def __init__(self, name: str, ty: IdlType, value: str) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def ty(self) -> IdlType: ...
    @property
    def value(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlConst", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlConst": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlConst": ...

class IdlField:
    def __init__(
        self, name: str, docs: Optional[Sequence[str]], ty: IdlType
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def docs(self) -> Optional[List[str]]: ...
    @property
    def ty(self) -> IdlType: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlField", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlField": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlField": ...

class IdlTypeDefStruct:
    def __init__(self, fields: Sequence[IdlField]) -> None: ...
    @property
    def fields(self) -> List[IdlField]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeDefStruct", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeDefStruct": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeDefStruct": ...

class IdlTypeDefAlias:
    def __init__(self, value: IdlType) -> None: ...
    @property
    def value(self) -> IdlType: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeDefAlias", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeDefAlias": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeDefAlias": ...

class EnumFieldsNamed:
    def __init__(self, fields: Sequence[IdlField]) -> None: ...
    @property
    def fields(self) -> List[IdlField]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "EnumFieldsNamed", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "EnumFieldsNamed": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "EnumFieldsNamed": ...

class EnumFieldsTuple:
    def __init__(self, fields: Sequence[IdlType]) -> None: ...
    @property
    def fields(self) -> List[IdlType]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "EnumFieldsTuple", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "EnumFieldsTuple": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "EnumFieldsTuple": ...

class IdlEnumVariant:
    def __init__(self, name: str, fields: Optional[EnumFields]) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def fields(self) -> Optional[EnumFields]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlEnumVariant", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlEnumVariant": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlEnumVariant": ...

class IdlTypeDefEnum:
    def __init__(self, variants: Sequence[IdlEnumVariant]) -> None: ...
    @property
    def variants(self) -> List[IdlEnumVariant]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeDefEnum", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeDefEnum": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeDefEnum": ...

IdlTypeDefinitionTy = Union[
    IdlTypeDefStruct, IdlTypeDefEnum, IdlTypeDefAlias
]

class IdlTypeDefinition:
    def __init__(
        self, name: str, docs: Optional[Sequence[str]], ty: IdlTypeDefinitionTy
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def docs(self) -> Optional[List[str]]: ...
    @property
    def ty(self) -> IdlTypeDefinitionTy: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlTypeDefinition", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlTypeDefinition": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlTypeDefinition": ...

IdlAccountItem = Union[IdlAccount, IdlAccounts]

class IdlAccounts:
    def __init__(self, name: str, accounts: Sequence[IdlAccountItem]) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def accounts(self) -> List[IdlAccountItem]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlAccounts", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlAccounts": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlAccounts": ...

class IdlSeedConst:
    def __init__(self, ty: IdlType, value: Json) -> None: ...
    @property
    def ty(self) -> IdlType: ...
    @property
    def value(
        self,
    ) -> Json: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlSeedConst", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlSeedConst": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlSeedConst": ...

class IdlSeedArg:
    def __init__(self, ty: IdlType, path: str) -> None: ...
    @property
    def ty(self) -> IdlType: ...
    @property
    def path(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlSeedArg", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlSeedArg": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlSeedArg": ...

class IdlSeedAccount:
    def __init__(self, ty: IdlType, account: Optional[str], path: str) -> None: ...
    @property
    def ty(self) -> IdlType: ...
    @property
    def acount(self) -> Optional[str]: ...
    @property
    def path(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlSeedAccount", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlSeedAccount": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlSeedAccount": ...

IdlSeed = Union[IdlSeedConst, IdlSeedArg, IdlSeedAccount]

class IdlPda:
    def __init__(
        self, seeds: Sequence[IdlSeed], program_id: Optional[IdlSeed]
    ) -> None: ...
    @property
    def seeds(self) -> List[IdlSeed]: ...
    @property
    def program_id(self) -> Optional[IdlSeed]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlPda", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlPda": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlPda": ...

class IdlAccount:
    def __init__(
        self,
        name: str,
        is_mut: bool,
        is_signer: bool,
        is_optional: Optional[bool],
        docs: Optional[Sequence[str]],
        pda: Optional[IdlPda],
        relations: Sequence[str],
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def is_mut(self) -> bool: ...
    @property
    def is_signer(self) -> bool: ...
    @property
    def is_optional(self) -> Optional[bool]: ...
    @property
    def docs(self) -> Optional[List[str]]: ...
    @property
    def pda(self) -> Optional[IdlPda]: ...
    @property
    def relations(self) -> List[str]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlAccount", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlAccount": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlAccount": ...

class IdlInstruction:
    def __init__(
        self,
        name: str,
        docs: Optional[Sequence[str]],
        accounts: Sequence[IdlAccountItem],
        args: Sequence[IdlField],
        returns: Optional[IdlType],
    ) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def docs(self) -> Optional[List[str]]: ...
    @property
    def accounts(self) -> List[IdlAccountItem]: ...
    @property
    def args(self) -> List[IdlField]: ...
    @property
    def returns(self) -> Optional[IdlType]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlInstruction", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlInstruction": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlInstruction": ...

class IdlState:
    def __init__(
        self, strct: IdlTypeDefinition, methods: Sequence[IdlInstruction]
    ) -> None: ...
    @property
    def strct(self) -> IdlTypeDefinition: ...
    @property
    def methods(self) -> List[IdlInstruction]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlState", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlState": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlState": ...

class IdlEvent:
    def __init__(self, name: str, fields: Sequence[IdlEventField]) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def fields(self) -> List[IdlEventField]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlEvent", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlEvent": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlEvent": ...

class IdlEventField:
    def __init__(self, name: str, ty: IdlType, index: bool) -> None: ...
    @property
    def name(self) -> str: ...
    @property
    def ty(self) -> IdlType: ...
    @property
    def index(self) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlEventField", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlEventField": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlEventField": ...

class IdlErrorCode:
    def __init__(self, code: int, name: str, msg: Optional[str]) -> None: ...
    @property
    def code(self) -> int: ...
    @property
    def name(self) -> str: ...
    @property
    def msg(self) -> Optional[str]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "IdlErrorCode", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "IdlErrorCode": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "IdlErrorCode": ...

class Idl:
    def __init__(
        self,
        version: str,
        name: str,
        docs: Optional[Sequence[str]],
        constants: Sequence[IdlConst],
        instructions: Sequence[IdlInstruction],
        state: Optional[IdlState],
        accounts: Sequence[IdlTypeDefinition],
        types: Sequence[IdlTypeDefinition],
        events: Optional[Sequence[IdlEvent]],
        errors: Optional[Sequence[IdlErrorCode]],
        metadata: Json = None,
    ) -> None: ...
    @property
    def version(self) -> str: ...
    @property
    def name(self) -> str: ...
    @property
    def docs(self) -> Optional[List[str]]: ...
    @property
    def constants(self) -> List[IdlConst]: ...
    @property
    def instructions(self) -> List[IdlInstruction]: ...
    @property
    def state(self) -> Optional[IdlState]: ...
    @property
    def accounts(self) -> List[IdlTypeDefinition]: ...
    @property
    def types(self) -> List[IdlTypeDefinition]: ...
    @property
    def events(self) -> Optional[List[IdlEvent]]: ...
    @property
    def errors(self) -> Optional[List[IdlErrorCode]]: ...
    @property
    def metadata(self) -> Json: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "Idl", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "Idl": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "Idl": ...

IdlTypeCompound = Union[
    IdlTypeDefined,
    IdlTypeOption,
    IdlTypeVec,
    IdlTypeArray,
    IdlTypeDefinedWithTypeArgs,
    IdlTypeGenericLenArray,
]
IdlType = Union[IdlTypeCompound, IdlTypeSimple]
EnumFields = Union[EnumFieldsNamed, EnumFieldsTuple]
