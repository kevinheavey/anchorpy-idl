use idl::{
    EnumFieldsNamed, EnumFieldsTuple, Idl, IdlAccount, IdlAccounts, IdlConst, IdlEnumVariant,
    IdlErrorCode, IdlEvent, IdlEventField, IdlField, IdlInstruction, IdlPda, IdlSeedAccount,
    IdlSeedArg, IdlSeedConst, IdlState, IdlTypeArray, IdlTypeDefined, IdlTypeDefinedWithTypeArgs,
    IdlTypeDefinition, IdlTypeDefinitionTyAlias, IdlTypeDefinitionTyEnum,
    IdlTypeDefinitionTyStruct, IdlTypeGeneric, IdlTypeGenericLenArray, IdlTypeOption,
    IdlTypeSimple, IdlTypeVec,
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
    m.add_class::<IdlTypeGenericLenArray>()?;
    m.add_class::<IdlTypeDefinedWithTypeArgs>()?;
    m.add_class::<IdlConst>()?;
    m.add_class::<IdlField>()?;
    m.add_class::<IdlTypeDefinitionTyStruct>()?;
    m.add_class::<EnumFieldsNamed>()?;
    m.add_class::<EnumFieldsTuple>()?;
    m.add_class::<IdlEnumVariant>()?;
    m.add_class::<IdlTypeDefinitionTyEnum>()?;
    m.add_class::<IdlTypeDefinitionTyAlias>()?;
    m.add_class::<IdlTypeDefinition>()?;
    m.add_class::<IdlAccounts>()?;
    m.add_class::<IdlSeedConst>()?;
    m.add_class::<IdlSeedArg>()?;
    m.add_class::<IdlSeedAccount>()?;
    m.add_class::<IdlPda>()?;
    m.add_class::<IdlAccount>()?;
    m.add_class::<IdlInstruction>()?;
    m.add_class::<IdlState>()?;
    m.add_class::<IdlEvent>()?;
    m.add_class::<IdlEventField>()?;
    m.add_class::<IdlErrorCode>()?;
    m.add_class::<IdlTypeGeneric>()?;
    m.add_class::<Idl>()?;

    let typing = py.import("typing")?;
    let union = typing.getattr("Union")?;
    let idl_account_item_members = vec![IdlAccount::type_object(py), IdlAccounts::type_object(py)];
    m.add(
        "IdlAccountItem",
        union.get_item(PyTuple::new(py, idl_account_item_members))?,
    )?;
    let idl_type_definition_ty_members = vec![
        IdlTypeDefinitionTyStruct::type_object(py),
        IdlTypeDefinitionTyEnum::type_object(py),
    ];
    m.add(
        "IdlTypeDefinitionTy",
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
    let compound_members = vec![
        IdlTypeDefined::type_object(py),
        IdlTypeOption::type_object(py),
        IdlTypeVec::type_object(py),
        IdlTypeArray::type_object(py),
        IdlTypeDefinedWithTypeArgs::type_object(py),
        IdlTypeGenericLenArray::type_object(py),
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
        EnumFieldsNamed::type_object(py),
        EnumFieldsTuple::type_object(py),
    ];
    m.add(
        "EnumFields",
        union.get_item(PyTuple::new(py, enum_fields_members))?,
    )?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
