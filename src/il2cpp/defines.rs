pub const IL2CPP_MAIN_MODULE : &'static str = "GameAssembly.dll\0";

pub const IL2CPP_INIT_EXPORT  : &'static str = "il2cpp_init\0";
pub const IL2CPP_CLASS_FROM_NAME_EXPORT  : &'static str = "il2cpp_class_from_name\0";
pub const IL2CPP_CLASS_GET_FIELDS_EXPORT  : &'static str = "il2cpp_class_get_fields\0";
pub const IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT  : &'static str = "il2cpp_class_get_field_from_name\0";
pub const IL2CPP_CLASS_GET_METHODS_EXPORT  : &'static str = "il2cpp_class_get_methods\0";
pub const IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT  : &'static str = "il2cpp_class_get_method_from_name\0";
pub const IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT  : &'static str = "il2cpp_class_get_property_from_name\0";
pub const IL2CPP_CLASS_GET_TYPE_EXPORT  : &'static str = "il2cpp_class_get_type\0";
pub const IL2CPP_DOMAIN_GET_EXPORT  : &'static str = "il2cpp_domain_get\0";
pub const IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT  : &'static str = "il2cpp_domain_get_assemblies\0";
pub const IL2CPP_FREE_EXPORT  : &'static str = "il2cpp_free\0";
pub const IL2CPP_IMAGE_GET_CLASS_EXPORT  : &'static str = "il2cpp_image_get_class\0";
pub const IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT  : &'static str = "il2cpp_image_get_class_count\0";
pub const IL2CPP_RESOLVE_FUNC_EXPORT  : &'static str = "il2cpp_resolve_icall\0";
pub const IL2CPP_STRING_NEW_EXPORT  : &'static str = "il2cpp_string_new\0";
pub const IL2CPP_THREAD_ATTACH_EXPORT  : &'static str = "il2cpp_thread_attach\0";
pub const IL2CPP_THREAD_DETACH_EXPORT  : &'static str = "il2cpp_thread_detach\0";
pub const IL2CPP_TYPE_GET_OBJECT_EXPORT  : &'static str = "il2cpp_type_get_object\0";

pub const IL2CPP_EXPORT_METHODS : [&str; 17] = [ 
    IL2CPP_CLASS_FROM_NAME_EXPORT,
    IL2CPP_CLASS_GET_FIELDS_EXPORT,
    IL2CPP_CLASS_GET_FIELD_FROM_NAME_EXPORT,
    IL2CPP_CLASS_GET_METHODS_EXPORT,
    IL2CPP_CLASS_GET_METHOD_FROM_NAME_EXPORT,
    IL2CPP_CLASS_GET_PROPERTY_FROM_NAME_EXPORT,
    IL2CPP_CLASS_GET_TYPE_EXPORT,
    IL2CPP_DOMAIN_GET_EXPORT,
    IL2CPP_DOMAIN_GET_ASSEMBLIES_EXPORT,
    IL2CPP_FREE_EXPORT,
    IL2CPP_IMAGE_GET_CLASS_EXPORT,
    IL2CPP_IMAGE_GET_CLASS_COUNT_EXPORT,
    IL2CPP_RESOLVE_FUNC_EXPORT,
    IL2CPP_STRING_NEW_EXPORT,
    IL2CPP_THREAD_ATTACH_EXPORT,
    IL2CPP_THREAD_DETACH_EXPORT,
    IL2CPP_TYPE_GET_OBJECT_EXPORT 
];
/*
pub enum Il2CppExportMethodIndex {
    Il2CppClassFromName = 0,
    Il2CppClassGetFields,
    Il2CppClassGetFieldFromName,
    Il2CppClassGetMethods,
    Il2CppClassGetMethodFromName,
    Il2CppClassGetPropertyFromName,
    Il2CppClassGetType,
    Il2CppDomainGet,
    Il2CppDomainGetAssemblies,
    Il2CppFree,
    Il2CppImageGetClass,
    Il2CppImageGetClassCount,
    Il2CppResolveICall,
    Il2CppStringNew,
    Il2CppThreadAttach,
    Il2CppThreadDetach,
    Il2CppTypeGetObject,
}*/