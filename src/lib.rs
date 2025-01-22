use idl::{
    Idl, IdlAccount, IdlConst, IdlDefinedFieldsNamed, IdlDefinedFieldsTuple, IdlDependency,
    IdlDeployments, IdlEnumVariant, IdlErrorCode, IdlEvent, IdlField, IdlInstruction,
    IdlInstructionAccount, IdlInstructionAccounts, IdlMetadata, IdlPda, IdlReprModifierC,
    IdlReprModifierRust, IdlReprSimple, IdlSeedAccount, IdlSeedArg, IdlSeedConst,
    IdlSerializationSimple, IdlTypeArray, IdlTypeDef, IdlTypeDefAlias, IdlTypeDefEnum,
    IdlTypeDefGenericConst, IdlTypeDefGenericType, IdlTypeDefStruct, IdlTypeDefined,
    IdlTypeGeneric, IdlTypeOption, IdlTypeSimple, IdlTypeVec,
};
use pyo3::{
    prelude::*,
    types::{PyString, PyTuple},
    PyTypeInfo,
};

pub mod idl;

#[pymodule]
fn anchorpy_idl(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<IdlTypeSimple>()?;
    m.add_class::<IdlTypeDefined>()?;
    m.add_class::<IdlTypeOption>()?;
    m.add_class::<IdlTypeVec>()?;
    m.add_class::<IdlTypeArray>()?;
    m.add_class::<IdlConst>()?;
    m.add_class::<IdlField>()?;
    m.add_class::<IdlTypeDefStruct>()?;
    m.add_class::<IdlDefinedFieldsNamed>()?;
    m.add_class::<IdlDefinedFieldsTuple>()?;
    m.add_class::<IdlEnumVariant>()?;
    m.add_class::<IdlTypeDefEnum>()?;
    m.add_class::<IdlTypeDefAlias>()?;
    m.add_class::<IdlTypeDefGenericConst>()?;
    m.add_class::<IdlTypeDefGenericType>()?;
    m.add_class::<IdlTypeDef>()?;
    m.add_class::<IdlInstructionAccounts>()?;
    m.add_class::<IdlSeedConst>()?;
    m.add_class::<IdlSeedArg>()?;
    m.add_class::<IdlReprModifierRust>()?;
    m.add_class::<IdlReprModifierC>()?;
    m.add_class::<IdlReprSimple>()?;
    m.add_class::<IdlSeedAccount>()?;
    m.add_class::<IdlSerializationSimple>()?;
    m.add_class::<IdlPda>()?;
    m.add_class::<IdlInstructionAccount>()?;
    m.add_class::<IdlInstruction>()?;
    m.add_class::<IdlEvent>()?;
    m.add_class::<IdlErrorCode>()?;
    m.add_class::<IdlTypeGeneric>()?;
    m.add_class::<IdlDependency>()?;
    m.add_class::<IdlDeployments>()?;
    m.add_class::<IdlMetadata>()?;
    m.add_class::<IdlAccount>()?;
    m.add_class::<Idl>()?;

    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let idl_account_item_members = vec![
        IdlInstructionAccount::type_object(py),
        IdlInstructionAccounts::type_object(py),
    ];
    m.add(
        "IdlAccountItem",
        union.get_item(PyTuple::new(py, idl_account_item_members))?,
    )?;
    let idl_type_definition_ty_members = vec![
        IdlTypeDefAlias::type_object(py),
        IdlTypeDefStruct::type_object(py),
        IdlTypeDefEnum::type_object(py),
    ];
    m.add(
        "IdlTypeDefTy",
        union.get_item(PyTuple::new(py, idl_type_definition_ty_members))?,
    )?;
    let idl_seed_members = vec![
        IdlSeedConst::type_object(py),
        IdlSeedArg::type_object(py),
        IdlSeedAccount::type_object(py),
    ];
    m.add(
        "IdlSeed",
        union.get_item(PyTuple::new(py, idl_seed_members))?,
    )?;

    let idl_repr_members = vec![
        IdlReprModifierRust::type_object(py),
        IdlReprModifierC::type_object(py),
        IdlReprSimple::type_object(py),
    ];
    m.add(
        "IdlRepr",
        union.get_item(PyTuple::new(py, idl_repr_members))?,
    )?;
    let idl_serialization_members = vec![
        IdlSerializationSimple::type_object(py),
        PyString::type_object(py),
    ];
    m.add(
        "IdlSerialization",
        union.get_item(PyTuple::new(py, idl_serialization_members))?,
    )?;
    let idl_type_def_generic_members = vec![
        IdlTypeDefGenericType::type_object(py),
        IdlTypeDefGenericConst::type_object(py),
    ];
    m.add(
        "IdlTypeDefGeneric",
        union.get_item(PyTuple::new(py, idl_type_def_generic_members))?,
    )?;
    let compound_members = vec![
        IdlTypeDefined::type_object(py),
        IdlTypeOption::type_object(py),
        IdlTypeVec::type_object(py),
        IdlTypeArray::type_object(py),
        PyString::type_object(py),
    ];
    m.add(
        "IdlTypeCompound",
        union.get_item(PyTuple::new(py, compound_members.clone()))?,
    )?;
    let mut idl_type_members = vec![IdlTypeSimple::type_object(py)];
    idl_type_members.extend(compound_members);
    m.add(
        "IdlType",
        union.get_item(PyTuple::new(py, idl_type_members.clone()))?,
    )?;
    let mut idl_defined_type_arg_members = idl_type_members;
    idl_defined_type_arg_members.extend(vec![
        IdlTypeGeneric::type_object(py),
        PyString::type_object(py),
    ]);
    m.add(
        "IdlDefinedTypeArg",
        union.get_item(PyTuple::new(py, idl_defined_type_arg_members))?,
    )?;
    let enum_fields_members = vec![
        IdlDefinedFieldsNamed::type_object(py),
        IdlDefinedFieldsTuple::type_object(py),
    ];
    m.add(
        "EnumFields",
        union.get_item(PyTuple::new(py, enum_fields_members))?,
    )?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
