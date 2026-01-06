//! ITypeInfo glue for automation calls.

mod helpers;
mod invoke;
mod methods;

use super::constants::OleMethod;

use invoke::typeinfo_invoke;
use methods::{
    typeinfo_add_ref, typeinfo_get_type_attr, typeinfo_not_impl, typeinfo_query_interface,
    typeinfo_release, typeinfo_release_type_attr,
};

pub(super) const TYPEINFO_METHODS: &[OleMethod] = &[
    ("pe_vm.typeinfo.QueryInterface", 3, typeinfo_query_interface),
    ("pe_vm.typeinfo.AddRef", 1, typeinfo_add_ref),
    ("pe_vm.typeinfo.Release", 1, typeinfo_release),
    ("pe_vm.typeinfo.GetTypeAttr", 2, typeinfo_get_type_attr),
    ("pe_vm.typeinfo.GetTypeComp", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetFuncDesc", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetVarDesc", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetNames", 5, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetRefTypeOfImplType", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetImplTypeFlags", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetIDsOfNames", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.Invoke", 8, typeinfo_invoke),
    ("pe_vm.typeinfo.GetDocumentation", 6, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetDllEntry", 6, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetRefTypeInfo", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.AddressOfMember", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.CreateInstance", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetMops", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetContainingTypeLib", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.ReleaseTypeAttr", 2, typeinfo_release_type_attr),
    ("pe_vm.typeinfo.ReleaseFuncDesc", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.ReleaseVarDesc", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetTypeKind", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetTypeFlags", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetFuncIndexOfMemId", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetVarIndexOfMemId", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetCustData", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetFuncCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetParamCustData", 5, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetVarCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetImplTypeCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetDocumentation2", 6, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllCustData", 2, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllFuncCustData", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllParamCustData", 4, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllVarCustData", 3, typeinfo_not_impl),
    ("pe_vm.typeinfo.GetAllImplTypeCustData", 3, typeinfo_not_impl),
];
