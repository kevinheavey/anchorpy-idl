use anchor_lang_idl_spec as anchor_idl;
use derive_more::{Display, From, Into};
use pyo3::prelude::*;
use serde::Deserialize;
use solders_macros::{pyhash, richcmp_eq_only};
use solders_traits::{handle_py_value_err, PyHash, RichcmpEqualityOnly};

trait PyFromJson<'a>: Deserialize<'a> {
    fn py_from_json(raw: &'a str) -> PyResult<Self> {
        let res: serde_json::Result<Self> = serde_json::from_str(raw);
        handle_py_value_err(res)
    }
}

macro_rules! struct_boilerplate {
    ($name:ident) => {
        impl RichcmpEqualityOnly for $name {}
        impl PyFromJson<'_> for $name {}
    };
}

macro_rules! debug_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

macro_rules! iter_into {
    ($obj:expr) => {
        $obj.into_iter().map(|x| x.into()).collect()
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, Hash)]
#[pyclass(module = "anchorpy_idl")]
pub enum IdlTypeSimple {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    U128,
    I128,
    U256,
    I256,
    Bytes,
    String,
    Pubkey,
}

impl From<IdlTypeSimple> for anchor_idl::IdlType {
    fn from(t: IdlTypeSimple) -> Self {
        match t {
            IdlTypeSimple::Bool => Self::Bool,
            IdlTypeSimple::U8 => Self::U8,
            IdlTypeSimple::I8 => Self::I8,
            IdlTypeSimple::U16 => Self::U16,
            IdlTypeSimple::I16 => Self::I16,
            IdlTypeSimple::U32 => Self::U32,
            IdlTypeSimple::I32 => Self::I32,
            IdlTypeSimple::F32 => Self::F32,
            IdlTypeSimple::U64 => Self::U64,
            IdlTypeSimple::I64 => Self::I64,
            IdlTypeSimple::F64 => Self::F64,
            IdlTypeSimple::U128 => Self::U128,
            IdlTypeSimple::I128 => Self::I128,
            IdlTypeSimple::U256 => Self::U256,
            IdlTypeSimple::I256 => Self::I256,
            IdlTypeSimple::Bytes => Self::Bytes,
            IdlTypeSimple::String => Self::String,
            IdlTypeSimple::Pubkey => Self::Pubkey,
        }
    }
}

impl PyHash for IdlTypeSimple {}

#[pyhash]
#[pymethods]
impl IdlTypeSimple {}

#[derive(Debug, Clone, PartialEq, Eq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDefined {
    #[serde(rename = "name")]
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    generics: Vec<IdlGenericArg>,
}

#[richcmp_eq_only]
#[pymethods]
impl IdlTypeDefined {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(name: String, generics: Vec<IdlGenericArg>) -> Self {
        Self {
            name: name,
            generics,
        }
    }
}

struct_boilerplate!(IdlTypeDefined);

#[derive(Debug, Clone, PartialEq, Eq, From, Into, Deserialize, Hash, Display)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeGeneric(String);

impl PyHash for IdlTypeGeneric {}

#[richcmp_eq_only]
#[pyhash]
#[pymethods]
impl IdlTypeGeneric {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(generic: String) -> Self {
        generic.into()
    }

    #[getter]
    pub fn generic(&self) -> String {
        self.0.clone()
    }
}

struct_boilerplate!(IdlTypeGeneric);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeOption(Box<IdlType>);

debug_display!(IdlTypeOption);

#[pymethods]
impl IdlTypeOption {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(option: IdlType) -> Self {
        Self(option.into())
    }

    #[getter]
    pub fn option(&self) -> IdlType {
        *self.0.clone()
    }
}

struct_boilerplate!(IdlTypeOption);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeVec(Box<IdlType>);

#[pymethods]
impl IdlTypeVec {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(vec: IdlType) -> Self {
        Self(vec.into())
    }

    #[getter]
    pub fn vec(&self) -> IdlType {
        *self.0.clone()
    }
}

struct_boilerplate!(IdlTypeVec);
debug_display!(IdlTypeVec);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlGenericArgType {
    #[serde(rename = "type")]
    #[pyo3(get)]
    pub ty: IdlType,
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlGenericArgConst {
    #[pyo3(get)]
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, FromPyObject, Eq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlGenericArg {
    Type(IdlGenericArgType),
    Const(IdlGenericArgConst),
}

impl IntoPy<PyObject> for IdlGenericArg {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Type(s) => s.into_py(py),
            Self::Const(v) => v.into_py(py),
        }
    }
}

impl From<IdlGenericArg> for anchor_idl::IdlGenericArg {
    fn from(value: IdlGenericArg) -> Self {
        match value {
            IdlGenericArg::Type(s) => Self::Type { ty: s.ty.into() },
            IdlGenericArg::Const(s) => Self::Const {
                value: s.value.into(),
            },
        }
    }
}

impl From<anchor_idl::IdlGenericArg> for IdlGenericArg {
    fn from(value: anchor_idl::IdlGenericArg) -> Self {
        match value {
            anchor_idl::IdlGenericArg::Type { ty } => {
                Self::Type(IdlGenericArgType { ty: ty.into() })
            }
            anchor_idl::IdlGenericArg::Const { value } => Self::Const(IdlGenericArgConst {
                value: value.into(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeArray(Box<IdlType>, IdlArrayLen);

#[pymethods]
impl IdlTypeArray {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(array: (IdlType, IdlArrayLen)) -> Self {
        Self(array.0.into(), array.1)
    }

    #[getter]
    pub fn array(&self, py: Python<'_>) -> (IdlType, PyObject) {
        (*self.0.clone(), self.clone().1.into_py(py))
    }
}

struct_boilerplate!(IdlTypeArray);
debug_display!(IdlTypeArray);

#[derive(Debug, Clone, Deserialize, PartialEq, FromPyObject, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IdlArrayLen {
    Generic(String),
    #[serde(untagged)]
    Value(usize),
}

impl IntoPy<PyObject> for IdlArrayLen {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Generic(s) => s.into_py(py),
            Self::Value(v) => v.into_py(py),
        }
    }
}

impl From<IdlArrayLen> for anchor_idl::IdlArrayLen {
    fn from(value: IdlArrayLen) -> Self {
        match value {
            IdlArrayLen::Generic(s) => Self::Generic(s),
            IdlArrayLen::Value(s) => Self::Value(s),
        }
    }
}

impl From<anchor_idl::IdlArrayLen> for IdlArrayLen {
    fn from(value: anchor_idl::IdlArrayLen) -> Self {
        match value {
            anchor_idl::IdlArrayLen::Generic(s) => Self::Generic(s),
            anchor_idl::IdlArrayLen::Value(s) => Self::Value(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromPyObject, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub enum IdlTypeCompound {
    Defined(IdlTypeDefined),
    Option(IdlTypeOption),
    Vec(IdlTypeVec),
    Array(IdlTypeArray),
    Generic(String),
}

impl From<IdlTypeCompound> for anchor_idl::IdlType {
    fn from(t: IdlTypeCompound) -> Self {
        match t {
            IdlTypeCompound::Defined(d) => Self::Defined {
                name: d.name,
                generics: d.generics.into_iter().map(|x| x.into()).collect(),
            },
            IdlTypeCompound::Option(o) => Self::Option(Box::new(Self::from(*o.0))),
            IdlTypeCompound::Vec(v) => Self::Vec(Box::new(Self::from(*v.0))),
            IdlTypeCompound::Array(a) => Self::Array(Box::new(Self::from(*a.0)), a.1.into()),
            IdlTypeCompound::Generic(g) => Self::Generic(g),
        }
    }
}

impl IntoPy<PyObject> for IdlTypeCompound {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlTypeCompound::Defined(x) => x.into_py(py),
            IdlTypeCompound::Option(x) => x.into_py(py),
            IdlTypeCompound::Vec(x) => x.into_py(py),
            IdlTypeCompound::Array(x) => x.into_py(py),
            IdlTypeCompound::Generic(x) => x.into_py(py),
        }
    }
}

#[derive(FromPyObject, Debug, Clone, PartialEq, Deserialize, Eq)]
#[serde(untagged)]
pub enum IdlType {
    Simple(IdlTypeSimple),
    Compound(IdlTypeCompound),
}

impl IntoPy<PyObject> for IdlType {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlType::Simple(s) => s.into_py(py),
            IdlType::Compound(c) => c.into_py(py),
        }
    }
}

impl From<anchor_idl::IdlType> for IdlType {
    fn from(t: anchor_idl::IdlType) -> Self {
        match t {
            anchor_idl::IdlType::Bool => Self::Simple(IdlTypeSimple::Bool),
            anchor_idl::IdlType::U8 => Self::Simple(IdlTypeSimple::U8),
            anchor_idl::IdlType::I8 => Self::Simple(IdlTypeSimple::I8),
            anchor_idl::IdlType::U16 => Self::Simple(IdlTypeSimple::U16),
            anchor_idl::IdlType::I16 => Self::Simple(IdlTypeSimple::I16),
            anchor_idl::IdlType::U32 => Self::Simple(IdlTypeSimple::U32),
            anchor_idl::IdlType::I32 => Self::Simple(IdlTypeSimple::I32),
            anchor_idl::IdlType::F32 => Self::Simple(IdlTypeSimple::F32),
            anchor_idl::IdlType::U64 => Self::Simple(IdlTypeSimple::U64),
            anchor_idl::IdlType::I64 => Self::Simple(IdlTypeSimple::I64),
            anchor_idl::IdlType::F64 => Self::Simple(IdlTypeSimple::F64),
            anchor_idl::IdlType::U128 => Self::Simple(IdlTypeSimple::U128),
            anchor_idl::IdlType::I128 => Self::Simple(IdlTypeSimple::I128),
            anchor_idl::IdlType::U256 => Self::Simple(IdlTypeSimple::U256),
            anchor_idl::IdlType::I256 => Self::Simple(IdlTypeSimple::I256),
            anchor_idl::IdlType::Bytes => Self::Simple(IdlTypeSimple::Bytes),
            anchor_idl::IdlType::String => Self::Simple(IdlTypeSimple::String),
            anchor_idl::IdlType::Pubkey => Self::Simple(IdlTypeSimple::Pubkey),
            anchor_idl::IdlType::Defined { name, generics } => {
                Self::Compound(IdlTypeCompound::Defined(IdlTypeDefined {
                    name,
                    generics: generics.into_iter().map(|x| x.into()).collect(),
                }))
            }
            anchor_idl::IdlType::Option(o) => Self::Compound(IdlTypeCompound::Option(
                IdlTypeOption(Box::new(IdlType::from(*o))),
            )),
            anchor_idl::IdlType::Vec(v) => Self::Compound(IdlTypeCompound::Vec(IdlTypeVec(
                Box::new(IdlType::from(*v)),
            ))),
            anchor_idl::IdlType::Array(a, size) => Self::Compound(IdlTypeCompound::Array(
                IdlTypeArray(Box::new(IdlType::from(*a)), size.into()),
            )),
            anchor_idl::IdlType::Generic(g) => Self::Compound(IdlTypeCompound::Generic(g)),
            _ => {
                panic!("Unexpected IdlType variant: {t:?}");
            }
        }
    }
}

impl From<IdlType> for anchor_idl::IdlType {
    fn from(t: IdlType) -> Self {
        match t {
            IdlType::Simple(s) => Self::from(s),
            IdlType::Compound(c) => Self::from(c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlConst(anchor_idl::IdlConst);

#[richcmp_eq_only]
#[pymethods]
impl IdlConst {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(name: String, ty: IdlType, value: String, docs: Vec<String>) -> Self {
        anchor_idl::IdlConst {
            name,
            ty: ty.into(),
            value,
            docs,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }

    #[getter]
    pub fn value(&self) -> String {
        self.0.value.clone()
    }
}

struct_boilerplate!(IdlConst);
debug_display!(IdlConst);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlField(anchor_idl::IdlField);

#[richcmp_eq_only]
#[pymethods]
impl IdlField {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(name: String, docs: Vec<String>, ty: IdlType) -> Self {
        anchor_idl::IdlField {
            name,
            docs,
            ty: ty.into(),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn docs(&self) -> Vec<String> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn ty(&self) -> IdlType {
        self.0.ty.clone().into()
    }
}

struct_boilerplate!(IdlField);
debug_display!(IdlField);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDefStruct {
    fields: Option<IdlDefinedFields>,
}

#[richcmp_eq_only]
#[pymethods]
impl IdlTypeDefStruct {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(fields: Option<IdlDefinedFields>) -> Self {
        Self { fields }
    }

    #[getter]
    pub fn fields(&self) -> Option<IdlDefinedFields> {
        self.fields.clone()
    }
}

struct_boilerplate!(IdlTypeDefStruct);
debug_display!(IdlTypeDefStruct);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDefAlias {
    alias: IdlType,
}

#[richcmp_eq_only]
#[pymethods]
impl IdlTypeDefAlias {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(alias: IdlType) -> Self {
        Self { alias }
    }

    #[getter]
    pub fn alias(&self) -> IdlType {
        self.alias.clone()
    }
}

struct_boilerplate!(IdlTypeDefAlias);
debug_display!(IdlTypeDefAlias);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlDefinedFieldsNamed(Vec<IdlField>);

#[richcmp_eq_only]
#[pymethods]
impl IdlDefinedFieldsNamed {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(fields: Vec<IdlField>) -> Self {
        fields.into()
    }

    #[getter]
    pub fn fields(&self) -> Vec<IdlField> {
        self.0.clone()
    }
}

struct_boilerplate!(IdlDefinedFieldsNamed);
debug_display!(IdlDefinedFieldsNamed);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlDefinedFieldsTuple(Vec<IdlType>);

#[richcmp_eq_only]
#[pymethods]
impl IdlDefinedFieldsTuple {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(fields: Vec<IdlType>) -> Self {
        fields.into()
    }

    #[getter]
    pub fn fields(&self) -> Vec<IdlType> {
        self.0.clone()
    }
}

struct_boilerplate!(IdlDefinedFieldsTuple);
debug_display!(IdlDefinedFieldsTuple);

#[derive(Debug, Clone, PartialEq, FromPyObject, Deserialize)]
#[serde(untagged)]
pub enum IdlDefinedFields {
    Named(IdlDefinedFieldsNamed),
    Tuple(IdlDefinedFieldsTuple),
}

impl From<IdlDefinedFields> for anchor_idl::IdlDefinedFields {
    fn from(t: IdlDefinedFields) -> Self {
        match t {
            IdlDefinedFields::Named(n) => Self::Named(iter_into!(n.0)),
            IdlDefinedFields::Tuple(t) => Self::Tuple(iter_into!(t.0)),
        }
    }
}

impl From<anchor_idl::IdlDefinedFields> for IdlDefinedFields {
    fn from(t: anchor_idl::IdlDefinedFields) -> Self {
        match t {
            anchor_idl::IdlDefinedFields::Named(n) => {
                Self::Named(IdlDefinedFieldsNamed(iter_into!(n)))
            }
            anchor_idl::IdlDefinedFields::Tuple(t) => {
                Self::Tuple(IdlDefinedFieldsTuple(iter_into!(t)))
            }
        }
    }
}

impl IntoPy<PyObject> for IdlDefinedFields {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlDefinedFields::Named(x) => x.into_py(py),
            IdlDefinedFields::Tuple(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlEnumVariant(anchor_idl::IdlEnumVariant);

#[richcmp_eq_only]
#[pymethods]
impl IdlEnumVariant {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(name: String, fields: Option<IdlDefinedFields>) -> Self {
        anchor_idl::IdlEnumVariant {
            name,
            fields: fields.map(|f| f.into()),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn fields(&self) -> Option<IdlDefinedFields> {
        self.0.fields.clone().map(|f| f.into())
    }
}

struct_boilerplate!(IdlEnumVariant);
debug_display!(IdlEnumVariant);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDefEnum {
    variants: Vec<IdlEnumVariant>,
}

#[richcmp_eq_only]
#[pymethods]
impl IdlTypeDefEnum {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(variants: Vec<IdlEnumVariant>) -> Self {
        Self { variants }
    }

    #[getter]
    pub fn variants(&self) -> Vec<IdlEnumVariant> {
        self.variants.clone()
    }
}

struct_boilerplate!(IdlTypeDefEnum);
debug_display!(IdlTypeDefEnum);

#[derive(Debug, Clone, PartialEq, FromPyObject, Deserialize)]
#[serde(rename_all = "lowercase", tag = "kind")]
pub enum IdlTypeDefTy {
    Struct(IdlTypeDefStruct),
    Enum(IdlTypeDefEnum),
    Type(IdlTypeDefAlias),
}

impl From<IdlTypeDefTy> for anchor_idl::IdlTypeDefTy {
    fn from(t: IdlTypeDefTy) -> Self {
        match t {
            IdlTypeDefTy::Struct(s) => Self::Struct {
                fields: s.fields.map(|x| x.into()),
            },
            IdlTypeDefTy::Enum(e) => Self::Enum {
                variants: e.variants.into_iter().map(|x| x.into()).collect(),
            },
            IdlTypeDefTy::Type(a) => Self::Type {
                alias: a.alias.into(),
            },
        }
    }
}

impl From<anchor_idl::IdlTypeDefTy> for IdlTypeDefTy {
    fn from(t: anchor_idl::IdlTypeDefTy) -> Self {
        match t {
            anchor_idl::IdlTypeDefTy::Struct { fields } => Self::Struct(IdlTypeDefStruct {
                fields: fields.map(|x| x.into()),
            }),
            anchor_idl::IdlTypeDefTy::Enum { variants } => Self::Enum(IdlTypeDefEnum {
                variants: variants.into_iter().map(|x| x.into()).collect(),
            }),
            anchor_idl::IdlTypeDefTy::Type { alias } => Self::Type(IdlTypeDefAlias {
                alias: alias.into(),
            }),
        }
    }
}

impl IntoPy<PyObject> for IdlTypeDefTy {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlTypeDefTy::Struct(x) => x.into_py(py),
            IdlTypeDefTy::Enum(x) => x.into_py(py),
            IdlTypeDefTy::Type(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, Hash, Default)]
#[pyclass(module = "anchorpy_idl")]
pub enum IdlSerializationSimple {
    #[default]
    Borsh,
    Bytemuck,
    BytemuckUnsafe,
}

impl From<IdlSerializationSimple> for anchor_idl::IdlSerialization {
    fn from(value: IdlSerializationSimple) -> Self {
        match value {
            IdlSerializationSimple::Borsh => Self::Borsh,
            IdlSerializationSimple::Bytemuck => Self::Bytemuck,
            IdlSerializationSimple::BytemuckUnsafe => Self::BytemuckUnsafe,
        }
    }
}

impl PyHash for IdlSerializationSimple {}

#[pyhash]
#[pymethods]
impl IdlSerializationSimple {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, Hash, FromPyObject)]
pub enum IdlSerializationCompound {
    Custom(String),
}

impl From<IdlSerializationCompound> for anchor_idl::IdlSerialization {
    fn from(t: IdlSerializationCompound) -> Self {
        match t {
            IdlSerializationCompound::Custom(s) => Self::Custom(s),
        }
    }
}

impl IntoPy<PyObject> for IdlSerializationCompound {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlSerializationCompound::Custom(x) => x.into_py(py),
        }
    }
}

#[derive(FromPyObject, Debug, Clone, PartialEq, Deserialize, Eq)]
#[serde(untagged)]
pub enum IdlSerialization {
    Simple(IdlSerializationSimple),
    Compound(IdlSerializationCompound),
}

impl IntoPy<PyObject> for IdlSerialization {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlSerialization::Simple(s) => s.into_py(py),
            IdlSerialization::Compound(c) => c.into_py(py),
        }
    }
}

impl From<IdlSerialization> for anchor_idl::IdlSerialization {
    fn from(t: IdlSerialization) -> Self {
        match t {
            IdlSerialization::Simple(s) => s.into(),
            IdlSerialization::Compound(s) => s.into(),
        }
    }
}

impl From<anchor_idl::IdlSerialization> for IdlSerialization {
    fn from(t: anchor_idl::IdlSerialization) -> Self {
        match t {
            anchor_idl::IdlSerialization::Borsh => Self::Simple(IdlSerializationSimple::Borsh),
            anchor_idl::IdlSerialization::Bytemuck => {
                Self::Simple(IdlSerializationSimple::Bytemuck)
            }
            anchor_idl::IdlSerialization::BytemuckUnsafe => {
                Self::Simple(IdlSerializationSimple::BytemuckUnsafe)
            }
            anchor_idl::IdlSerialization::Custom(s) => {
                Self::Compound(IdlSerializationCompound::Custom(s))
            }
            _ => {
                panic!("Unexpected IdlSerialization variant: {t:?}");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDefGenericType {
    #[pyo3(get)]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize, Eq)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDefGenericConst {
    #[serde(rename = "name")]
    #[pyo3(get)]
    name: String,
    #[serde(rename = "type")]
    #[pyo3(get)]
    ty: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, FromPyObject, Eq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlTypeDefGeneric {
    Type(IdlTypeDefGenericType),
    Const(IdlTypeDefGenericConst),
}

impl IntoPy<PyObject> for IdlTypeDefGeneric {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Type(s) => s.into_py(py),
            Self::Const(v) => v.into_py(py),
        }
    }
}

impl From<IdlTypeDefGeneric> for anchor_idl::IdlTypeDefGeneric {
    fn from(value: IdlTypeDefGeneric) -> Self {
        match value {
            IdlTypeDefGeneric::Type(s) => Self::Type { name: s.name },
            IdlTypeDefGeneric::Const(s) => Self::Const {
                name: s.name,
                ty: s.ty,
            },
        }
    }
}

impl From<anchor_idl::IdlTypeDefGeneric> for IdlTypeDefGeneric {
    fn from(value: anchor_idl::IdlTypeDefGeneric) -> Self {
        match value {
            anchor_idl::IdlTypeDefGeneric::Type { name } => {
                Self::Type(IdlTypeDefGenericType { name })
            }
            anchor_idl::IdlTypeDefGeneric::Const { name, ty } => {
                Self::Const(IdlTypeDefGenericConst { name, ty })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlReprModifierRust(anchor_idl::IdlReprModifier);

#[pymethods]
impl IdlReprModifierRust {
    #[new]
    pub fn new(packed: bool, align: Option<usize>) -> Self {
        Self(anchor_idl::IdlReprModifier { packed, align })
    }

    #[getter]
    pub fn packed(&self) -> bool {
        self.0.packed
    }

    #[getter]
    pub fn align(&self) -> Option<usize> {
        self.0.align
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlReprModifierC(anchor_idl::IdlReprModifier);

#[pymethods]
impl IdlReprModifierC {
    #[new]
    pub fn new(packed: bool, align: Option<usize>) -> Self {
        Self(anchor_idl::IdlReprModifier { packed, align })
    }

    #[getter]
    pub fn packed(&self) -> bool {
        self.0.packed
    }

    #[getter]
    pub fn align(&self) -> Option<usize> {
        self.0.align
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, Hash)]
#[pyclass(module = "anchorpy_idl")]
pub enum IdlReprSimple {
    Transparent,
}

impl From<IdlReprSimple> for anchor_idl::IdlRepr {
    fn from(value: IdlReprSimple) -> Self {
        match value {
            IdlReprSimple::Transparent => Self::Transparent,
        }
    }
}

impl PyHash for IdlReprSimple {}

#[pyhash]
#[pymethods]
impl IdlReprSimple {}

#[derive(Debug, Clone, PartialEq, Deserialize, FromPyObject)]
pub enum IdlReprCompound {
    Rust(IdlReprModifierRust),
    C(IdlReprModifierC),
}

impl From<IdlReprCompound> for anchor_idl::IdlRepr {
    fn from(t: IdlReprCompound) -> Self {
        match t {
            IdlReprCompound::Rust(x) => Self::Rust(x.0),
            IdlReprCompound::C(x) => Self::C(x.0),
        }
    }
}

impl IntoPy<PyObject> for IdlReprCompound {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlReprCompound::Rust(x) => x.into_py(py),
            IdlReprCompound::C(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, FromPyObject)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum IdlRepr {
    Compound(IdlReprCompound),
    Simple(IdlReprSimple),
}

impl IntoPy<PyObject> for IdlRepr {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Self::Compound(s) => s.into_py(py),
            Self::Simple(v) => v.into_py(py),
        }
    }
}

impl From<IdlRepr> for anchor_idl::IdlRepr {
    fn from(value: IdlRepr) -> Self {
        match value {
            IdlRepr::Compound(s) => s.into(),
            IdlRepr::Simple(s) => s.into(),
        }
    }
}

impl From<anchor_idl::IdlRepr> for IdlRepr {
    fn from(value: anchor_idl::IdlRepr) -> Self {
        match value {
            anchor_idl::IdlRepr::Rust(x) => Self::Compound(IdlReprCompound::Rust(x.into())),
            anchor_idl::IdlRepr::C(x) => Self::Compound(IdlReprCompound::C(x.into())),
            anchor_idl::IdlRepr::Transparent => Self::Simple(IdlReprSimple::Transparent),
            _ => {
                panic!("Unexpected IdlRepr variant: {value:?}");
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlTypeDef(anchor_idl::IdlTypeDef);

#[richcmp_eq_only]
#[pymethods]
impl IdlTypeDef {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(
        name: String,
        docs: Vec<String>,
        serialization: IdlSerialization,
        repr: Option<IdlRepr>,
        generics: Vec<IdlTypeDefGeneric>,
        ty: IdlTypeDefTy,
    ) -> Self {
        anchor_idl::IdlTypeDef {
            name,
            docs,
            serialization: serialization.into(),
            repr: repr.map(|x| x.into()),
            generics: generics.into_iter().map(|x| x.into()).collect(),
            ty: ty.into(),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn docs(&self) -> Vec<String> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn serialization(&self) -> IdlSerialization {
        self.0.serialization.clone().into()
    }

    #[getter]
    pub fn generics(&self) -> Vec<IdlTypeDefGeneric> {
        self.0
            .generics
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }

    #[getter]
    pub fn ty(&self) -> IdlTypeDefTy {
        self.0.ty.clone().into()
    }
}

struct_boilerplate!(IdlTypeDef);
debug_display!(IdlTypeDef);

#[derive(Debug, Clone, Deserialize, PartialEq, FromPyObject)]
#[serde(untagged)]
pub enum IdlInstructionAccountItem {
    Composite(IdlInstructionAccounts),
    Single(IdlInstructionAccount),
}

impl From<IdlInstructionAccountItem> for anchor_idl::IdlInstructionAccountItem {
    fn from(a: IdlInstructionAccountItem) -> Self {
        match a {
            IdlInstructionAccountItem::Single(x) => Self::Single(x.into()),
            IdlInstructionAccountItem::Composite(x) => Self::Composite(x.into()),
        }
    }
}

impl From<anchor_idl::IdlInstructionAccountItem> for IdlInstructionAccountItem {
    fn from(a: anchor_idl::IdlInstructionAccountItem) -> Self {
        match a {
            anchor_idl::IdlInstructionAccountItem::Single(x) => Self::Single(x.into()),
            anchor_idl::IdlInstructionAccountItem::Composite(x) => Self::Composite(x.into()),
        }
    }
}

impl IntoPy<PyObject> for IdlInstructionAccountItem {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlInstructionAccountItem::Single(x) => x.into_py(py),
            IdlInstructionAccountItem::Composite(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlInstructionAccounts(anchor_idl::IdlInstructionAccounts);

#[richcmp_eq_only]
#[pymethods]
impl IdlInstructionAccounts {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(name: String, accounts: Vec<IdlInstructionAccountItem>) -> Self {
        anchor_idl::IdlInstructionAccounts {
            name,
            accounts: iter_into!(accounts),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn accounts(&self) -> Vec<IdlInstructionAccountItem> {
        iter_into!(self.0.accounts.clone())
    }
}

struct_boilerplate!(IdlInstructionAccounts);
debug_display!(IdlInstructionAccounts);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlSeedConst(anchor_idl::IdlSeedConst);

#[pymethods]
impl IdlSeedConst {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(value: Vec<u8>) -> Self {
        Self(anchor_idl::IdlSeedConst { value })
    }

    #[getter]
    pub fn value(&self) -> Vec<u8> {
        self.0.value.clone()
    }
}

struct_boilerplate!(IdlSeedConst);
debug_display!(IdlSeedConst);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlSeedArg(anchor_idl::IdlSeedArg);

#[richcmp_eq_only]
#[pymethods]
impl IdlSeedArg {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(path: String) -> Self {
        anchor_idl::IdlSeedArg { path }.into()
    }

    #[getter]
    pub fn path(&self) -> String {
        self.0.path.clone()
    }
}

struct_boilerplate!(IdlSeedArg);
debug_display!(IdlSeedArg);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlSeedAccount(anchor_idl::IdlSeedAccount);

#[richcmp_eq_only]
#[pymethods]
impl IdlSeedAccount {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(account: Option<String>, path: String) -> Self {
        anchor_idl::IdlSeedAccount { account, path }.into()
    }

    #[getter]
    pub fn acount(&self) -> Option<String> {
        self.0.account.clone()
    }

    #[getter]
    pub fn path(&self) -> String {
        self.0.path.clone()
    }
}

struct_boilerplate!(IdlSeedAccount);
debug_display!(IdlSeedAccount);

#[derive(Debug, Clone, Deserialize, PartialEq, FromPyObject)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum IdlSeed {
    Const(IdlSeedConst),
    Arg(IdlSeedArg),
    Account(IdlSeedAccount),
}

impl From<IdlSeed> for anchor_idl::IdlSeed {
    fn from(s: IdlSeed) -> Self {
        match s {
            IdlSeed::Const(x) => Self::Const(x.into()),
            IdlSeed::Arg(x) => Self::Arg(x.into()),
            IdlSeed::Account(x) => Self::Account(x.into()),
        }
    }
}

impl From<anchor_idl::IdlSeed> for IdlSeed {
    fn from(s: anchor_idl::IdlSeed) -> Self {
        match s {
            anchor_idl::IdlSeed::Const(x) => Self::Const(x.into()),
            anchor_idl::IdlSeed::Arg(x) => Self::Arg(x.into()),
            anchor_idl::IdlSeed::Account(x) => Self::Account(x.into()),
        }
    }
}

impl IntoPy<PyObject> for IdlSeed {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            IdlSeed::Const(x) => x.into_py(py),
            IdlSeed::Arg(x) => x.into_py(py),
            IdlSeed::Account(x) => x.into_py(py),
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlPda(anchor_idl::IdlPda);

#[richcmp_eq_only]
#[pymethods]
impl IdlPda {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(seeds: Vec<IdlSeed>, program: Option<IdlSeed>) -> Self {
        anchor_idl::IdlPda {
            seeds: iter_into!(seeds),
            program: program.map(|x| x.into()),
        }
        .into()
    }

    #[getter]
    pub fn seeds(&self) -> Vec<IdlSeed> {
        iter_into!(self.0.seeds.clone())
    }

    #[getter]
    pub fn program(&self) -> Option<IdlSeed> {
        self.0.program.clone().map(|x| x.into())
    }
}

struct_boilerplate!(IdlPda);
debug_display!(IdlPda);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlInstructionAccount(anchor_idl::IdlInstructionAccount);

#[richcmp_eq_only]
#[pymethods]
impl IdlInstructionAccount {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(
        name: String,
        docs: Vec<String>,
        writable: bool,
        signer: bool,
        optional: bool,
        address: Option<String>,
        pda: Option<IdlPda>,
        relations: Vec<String>,
    ) -> Self {
        anchor_idl::IdlInstructionAccount {
            name,
            writable,
            signer,
            optional,
            address,
            docs,
            pda: pda.map(|x| x.into()),
            relations,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn writable(&self) -> bool {
        self.0.writable
    }

    #[getter]
    pub fn signer(&self) -> bool {
        self.0.signer
    }

    #[getter]
    pub fn optional(&self) -> bool {
        self.0.optional
    }

    #[getter]
    pub fn docs(&self) -> Vec<String> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn address(&self) -> Option<String> {
        self.0.address.clone()
    }

    #[getter]
    pub fn pda(&self) -> Option<IdlPda> {
        self.0.pda.clone().map(|x| x.into())
    }

    #[getter]
    pub fn relations(&self) -> Vec<String> {
        self.0.relations.clone()
    }
}

struct_boilerplate!(IdlInstructionAccount);
debug_display!(IdlInstructionAccount);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlInstruction(anchor_idl::IdlInstruction);

#[richcmp_eq_only]
#[pymethods]
impl IdlInstruction {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(
        name: String,
        docs: Vec<String>,
        discriminator: Vec<u8>,
        accounts: Vec<IdlInstructionAccountItem>,
        args: Vec<IdlField>,
        returns: Option<IdlType>,
    ) -> Self {
        anchor_idl::IdlInstruction {
            name,
            docs,
            discriminator,
            accounts: iter_into!(accounts),
            args: iter_into!(args),
            returns: returns.map(|x| x.into()),
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn docs(&self) -> Vec<String> {
        self.0.docs.clone()
    }

    #[getter]
    pub fn discriminator(&self) -> Vec<u8> {
        self.0.discriminator.clone()
    }

    #[getter]
    pub fn accounts(&self) -> Vec<IdlInstructionAccountItem> {
        iter_into!(self.0.accounts.clone())
    }

    #[getter]
    pub fn args(&self) -> Vec<IdlField> {
        iter_into!(self.0.args.clone())
    }

    #[getter]
    pub fn returns(&self) -> Option<IdlType> {
        self.0.returns.clone().map(|x| x.into())
    }
}

struct_boilerplate!(IdlInstruction);
debug_display!(IdlInstruction);
#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlEvent(anchor_idl::IdlEvent);

#[richcmp_eq_only]
#[pymethods]
impl IdlEvent {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(name: String, discriminator: Vec<u8>) -> Self {
        anchor_idl::IdlEvent {
            name,
            discriminator,
        }
        .into()
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn discriminator(&self) -> Vec<u8> {
        self.0.discriminator.clone()
    }
}

struct_boilerplate!(IdlEvent);
debug_display!(IdlEvent);

#[derive(Debug, Clone, PartialEq, Eq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlErrorCode(anchor_idl::IdlErrorCode);

#[richcmp_eq_only]
#[pymethods]
impl IdlErrorCode {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(code: u32, name: String, msg: Option<String>) -> Self {
        anchor_idl::IdlErrorCode { code, name, msg }.into()
    }

    #[getter]
    pub fn code(&self) -> u32 {
        self.0.code
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn msg(&self) -> Option<String> {
        self.0.msg.clone()
    }
}

struct_boilerplate!(IdlErrorCode);
debug_display!(IdlErrorCode);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct Idl(anchor_idl::Idl);

#[pymethods]
impl Idl {
    #[allow(clippy::too_many_arguments)]
    #[new]
    pub fn new(
        address: String,
        metadata: IdlMetadata,
        docs: Vec<String>,
        instructions: Vec<IdlInstruction>,
        accounts: Vec<IdlAccount>,
        events: Vec<IdlEvent>,
        errors: Vec<IdlErrorCode>,
        types: Vec<IdlTypeDef>,
        constants: Vec<IdlConst>,
    ) -> Self {
        Self(anchor_idl::Idl {
            address,
            metadata: metadata.0,
            docs,
            instructions: iter_into!(instructions),
            accounts: iter_into!(accounts),
            events: iter_into!(events),
            errors: iter_into!(errors),
            types: iter_into!(types),
            constants: iter_into!(constants),
        })
    }

    #[getter]
    pub fn address(&self) -> String {
        self.0.address.clone()
    }
    #[getter]
    pub fn metadata(&self) -> IdlMetadata {
        self.0.metadata.clone().into()
    }
    #[getter]
    pub fn docs(&self) -> Vec<String> {
        self.0.docs.clone()
    }
    #[getter]
    pub fn instructions(&self) -> Vec<IdlInstruction> {
        iter_into!(self.0.instructions.clone())
    }
    #[getter]
    pub fn accounts(&self) -> Vec<IdlAccount> {
        iter_into!(self.0.accounts.clone())
    }
    #[getter]
    pub fn events(&self) -> Vec<IdlEvent> {
        iter_into!(self.0.events.clone())
    }
    #[getter]
    pub fn errors(&self) -> Vec<IdlErrorCode> {
        iter_into!(self.0.errors.clone())
    }
    #[getter]
    pub fn types(&self) -> Vec<IdlTypeDef> {
        iter_into!(self.0.types.clone())
    }
    #[getter]
    pub fn constants(&self) -> Vec<IdlConst> {
        iter_into!(self.0.constants.clone())
    }

    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
}

struct_boilerplate!(Idl);
debug_display!(Idl);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlDeployments(anchor_idl::IdlDeployments);

#[pymethods]
impl IdlDeployments {
    #[new]
    pub fn new(
        mainnet: Option<String>,
        testnet: Option<String>,
        devnet: Option<String>,
        localnet: Option<String>,
    ) -> Self {
        Self(anchor_idl::IdlDeployments {
            mainnet,
            testnet,
            devnet,
            localnet,
        })
    }

    #[getter]
    pub fn mainnet(&self) -> Option<String> {
        self.0.mainnet.clone()
    }

    #[getter]
    pub fn testnet(&self) -> Option<String> {
        self.0.testnet.clone()
    }

    #[getter]
    pub fn devnet(&self) -> Option<String> {
        self.0.devnet.clone()
    }

    #[getter]
    pub fn localnet(&self) -> Option<String> {
        self.0.localnet.clone()
    }

    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
}

struct_boilerplate!(IdlDeployments);
debug_display!(IdlDeployments);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlDependency(anchor_idl::IdlDependency);

#[pymethods]
impl IdlDependency {
    #[new]
    pub fn new(name: String, version: String) -> Self {
        Self(anchor_idl::IdlDependency { name, version })
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn version(&self) -> String {
        self.0.version.clone()
    }
}

struct_boilerplate!(IdlDependency);
debug_display!(IdlDependency);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlMetadata(anchor_idl::IdlMetadata);

#[pymethods]
impl IdlMetadata {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }
    #[new]
    pub fn new(
        name: String,
        version: String,
        spec: String,
        description: Option<String>,
        repository: Option<String>,
        dependencies: Vec<IdlDependency>,
        contact: Option<String>,
        deployments: Option<IdlDeployments>,
    ) -> Self {
        Self(anchor_idl::IdlMetadata {
            name,
            version,
            spec,
            description,
            repository,
            dependencies: iter_into!(dependencies),
            contact,
            deployments: deployments.map(|x| x.into()),
        })
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn version(&self) -> String {
        self.0.version.clone()
    }

    #[getter]
    pub fn spec(&self) -> String {
        self.0.spec.clone()
    }

    #[getter]
    pub fn description(&self) -> Option<String> {
        self.0.description.clone()
    }

    #[getter]
    pub fn repository(&self) -> Option<String> {
        self.0.repository.clone()
    }

    #[getter]
    pub fn dependencies(&self) -> Vec<IdlDependency> {
        self.0
            .dependencies
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }

    #[getter]
    pub fn contact(&self) -> Option<String> {
        self.0.contact.clone()
    }

    #[getter]
    pub fn deployments(&self) -> Vec<IdlDeployments> {
        self.0
            .deployments
            .clone()
            .into_iter()
            .map(|x| x.into())
            .collect()
    }
}

struct_boilerplate!(IdlMetadata);
debug_display!(IdlMetadata);

#[derive(Debug, Clone, PartialEq, From, Into, Deserialize)]
#[pyclass(module = "anchorpy_idl", subclass)]
pub struct IdlAccount(anchor_idl::IdlAccount);

#[pymethods]
impl IdlAccount {
    #[staticmethod]
    pub fn from_json(raw: &str) -> PyResult<Self> {
        Self::py_from_json(raw)
    }

    #[new]
    pub fn new(name: String, discriminator: Vec<u8>) -> Self {
        Self(anchor_idl::IdlAccount {
            name,
            discriminator,
        })
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    pub fn discriminator(&self) -> Vec<u8> {
        self.0.discriminator.clone()
    }
}

struct_boilerplate!(IdlAccount);
debug_display!(IdlAccount);
