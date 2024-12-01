use proc_macro2::Ident;
use quote::format_ident;
use syn::{Field, Type};

pub fn create_builder_ident(name: &Ident) -> Ident {
    format_ident!("{}Builder", name)
}

pub fn create_field_struct_name(builder: &Ident, field: &Ident) -> Ident {
    format_ident!("{}Of{}", field, builder)
}

pub fn get_name_and_type(f: &Field) -> (&Option<Ident>, &Type) {
    let field_name = &f.ident;
    let field_ty = &f.ty;
    (field_name, field_ty)
}
