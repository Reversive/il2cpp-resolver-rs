use windows::core::PCSTR;

const RAW_IL2CPP_MAIN_MODULE_DLL : &'static [u8] = b"GameAssembly.dll\0";
pub const IL2CPP_MAIN_MODULE : PCSTR = PCSTR::from_raw(RAW_IL2CPP_MAIN_MODULE_DLL.as_ptr());

const RAW_IL2CPP_INIT_EXPORT : &'static [u8] = b"il2cpp_init\0";
pub const IL2CPP_INIT_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_INIT_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_FROM_NAME_EXPORT : &'static [u8] = b"il2cpp_class_from_name\0";
pub const IL2CPP_CLASS_FROM_NAME_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_FROM_NAME_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_GET_FIELDS_EXPORT : &'static [u8] = b"il2cpp_class_get_fields\0";
pub const IL2CPP_CLASS_GET_FIELDS_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_GET_FIELDS_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT : &'static [u8] = b"il2cpp_class_get_field_from_name\0";
pub const IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_GET_METHODS_EXPORT : &'static [u8] = b"il2cpp_class_get_methods\0";
pub const IL2CPP_CLASS_GET_METHODS_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_GET_METHODS_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT : &'static [u8] = b"il2cpp_class_get_method_from_name\0";
pub const IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT : &'static [u8] = b"il2cpp_class_get_property_from_name\0";
pub const IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT.as_ptr());

const RAW_IL2CPP_CLASS_GET_TYPE_EXPORT : &'static [u8] = b"il2cpp_class_get_type\0";
pub const IL2CPP_CLASS_GET_TYPE_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_CLASS_GET_TYPE_EXPORT.as_ptr());

const RAW_IL2CPP_DOMAIN_GET_EXPORT : &'static [u8] = b"il2cpp_domain_get\0";
pub const IL2CPP_DOMAIN_GET_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_DOMAIN_GET_EXPORT.as_ptr());

const RAW_IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT : &'static [u8] = b"il2cpp_domain_get_assemblies\0";
pub const IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT.as_ptr());

const RAW_IL2CPP_FREE_EXPORT : &'static [u8] = b"il2cpp_free\0";
pub const IL2CPP_FREE_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_FREE_EXPORT.as_ptr());

const RAW_IL2CPP_IMAGE_GET_CLASS_EXPORT : &'static [u8] = b"il2cpp_image_get_class\0";
pub const IL2CPP_IMAGE_GET_CLASS_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_IMAGE_GET_CLASS_EXPORT.as_ptr());

const RAW_IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT : &'static [u8] = b"il2cpp_image_get_class_count\0";
pub const IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT.as_ptr());

const RAW_IL2CPP_RESOLVE_FUNC_EXPORT : &'static [u8] = b"il2cpp_resolve_icall\0";
pub const IL2CPP_RESOLVE_FUNC_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_RESOLVE_FUNC_EXPORT.as_ptr());

const RAW_IL2CPP_STRING_NEW_EXPORT : &'static [u8] = b"il2cpp_string_new\0";
pub const IL2CPP_STRING_NEW_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_STRING_NEW_EXPORT.as_ptr());

const RAW_IL2CPP_THREAD_ATTACH_EXPORT : &'static [u8] = b"il2cpp_thread_attach\0";
pub const IL2CPP_THREAD_ATTACH_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_THREAD_ATTACH_EXPORT.as_ptr());

const RAW_IL2CPP_THREAD_DETACH_EXPORT : &'static [u8] = b"il2cpp_thread_detach\0";
pub const IL2CPP_THREAD_DETACH_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_THREAD_DETACH_EXPORT.as_ptr());

const RAW_IL2CPP_TYPE_GET_OBJECT_EXPORT : &'static [u8] = b"il2cpp_type_get_object\0";
pub const IL2CPP_TYPE_GET_OBJECT_EXPORT : PCSTR = PCSTR::from_raw(RAW_IL2CPP_TYPE_GET_OBJECT_EXPORT.as_ptr());