use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident};

use crate::utils::{create_builder_ident, create_field_struct_name, get_name_and_type};

fn original_struct_setters<FS: FallbackStrategy>(
    fields: &Punctuated<Field, Comma>,
    fallback_strategy: FS,
) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let field_name = &f.ident;
            let field_name_as_string = field_name.as_ref().unwrap().to_string();

            let handle_missing = fallback_strategy.fallback(field_name_as_string);

            quote! {
                #field_name: self.#field_name.#handle_missing
            }
        })
        .collect()
}

pub fn marker_trait_and_structs(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_name = create_builder_ident(name);

    let structs_and_impls = fields.iter().map(|f| {
        let field_name = &f.ident.clone().unwrap();
        let struct_name = create_field_struct_name(&builder_name, field_name);

        quote! {
            #[allow(non_camel_case_types)]
            pub struct #struct_name {}
            impl MarkerTraitForBuilder for #struct_name {}
        }
    });

    quote! {
        pub trait MarkerTraitForBuilder {}

        #(#structs_and_impls)*

        pub struct FinalBuilder {}
        impl MarkerTraitForBuilder for FinalBuilder {}
    }
}

pub fn builder_definition(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_fields = fields.iter().map(|f| {
        let (field_name, field_ty) = get_name_and_type(f);
        quote! { #field_name: Option<#field_ty> }
    });

    let builder_name = create_builder_ident(name);

    quote! {
        pub struct #builder_name<T: MarkerTraitForBuilder> {
            marker: std::marker::PhantomData<T>,
            #(#builder_fields,)*
        }
    }
}

pub fn builder_impl_for_struct(name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    });

    let first_field_name = fields.first().map(|f| f.ident.clone().unwrap()).unwrap();

    let builder_name = create_builder_ident(name);
    let generic = create_field_struct_name(&builder_name, &first_field_name);

    quote! {
        impl #name {
            pub fn builder() -> #builder_name<#generic> {
                #builder_name {
                    marker: Default::default(),
                    #(#builder_inits,)*
                }
            }
        }
    }
}

pub fn builder_methods(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_name = create_builder_ident(struct_name);
    let set_fields = original_struct_setters(fields, ConcreteFallbackStrategy::Panic);
    let assignments_for_all_fields = get_assignments_for_fields(fields);
    let mut previous_field = None;
    let reversed_names_and_types: Vec<&Field> = fields.iter().rev().collect();

    let methods: Vec<TokenStream> = reversed_names_and_types
        .iter()
        .map(|f| {
            if let Some(next_in_list) = previous_field {
                previous_field = Some(f);
                builder_for_field(&builder_name, &assignments_for_all_fields, f, next_in_list)
            } else {
                previous_field = Some(f);
                builder_for_final_field(&builder_name, &assignments_for_all_fields, f)
            }
        })
        .collect();

    quote! {
        #(#methods)*

        impl #builder_name<FinalBuilder> {
            pub fn build(self) -> #struct_name {
                #struct_name {
                    #(#set_fields,)*
                }
            }

        }
    }
}

fn get_assignments_for_fields(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let (f_name, _) = get_name_and_type(f);
            let field = f_name.as_ref().unwrap();
            quote! {
                #field: self.#field
            }
        })
        .collect()
}

fn builder_for_field(
    builder_name: &Ident,
    field_assignments: &Vec<TokenStream>,
    current_field: &Field,
    next_field_in_list: &Field,
) -> TokenStream {
    let (field_name, field_ty) = get_name_and_type(current_field);
    let (next_field_name, _) = get_name_and_type(next_field_in_list);
    let current_field_struct_name =
        create_field_struct_name(builder_name, field_name.as_ref().unwrap());
    let next_field_struct_name =
        create_field_struct_name(builder_name, next_field_name.as_ref().unwrap());

    quote! {
        impl #builder_name<#current_field_struct_name> {
            pub fn #field_name(mut self, input: #field_ty) -> #builder_name<#next_field_struct_name> {
                self.#field_name = Some(input);
                #builder_name {
                    marker: Default::default(),
                    #(#field_assignments,)*
                }
            }
        }
    }
}

fn builder_for_final_field(
    builder_name: &Ident,
    field_assignments: &Vec<TokenStream>,
    field: &Field,
) -> TokenStream {
    let (field_name, field_ty) = get_name_and_type(field);
    let field_struct_name = create_field_struct_name(builder_name, field_name.as_ref().unwrap());

    quote! {
        impl #builder_name<#field_struct_name> {
            pub fn #field_name(mut self, input: #field_ty) -> #builder_name<FinalBuilder> {
                self.#field_name = Some(input);
                #builder_name {
                    marker: Default::default(),
                    #(#field_assignments,)*
                }
            }
        }
    }
}

trait FallbackStrategy {
    fn fallback(&self, field_name_as_string: String) -> TokenStream;
}

#[allow(dead_code)]
enum ConcreteFallbackStrategy {
    Default,
    Panic,
}

impl FallbackStrategy for ConcreteFallbackStrategy {
    fn fallback(&self, field_name_as_string: String) -> TokenStream {
        match self {
            ConcreteFallbackStrategy::Default => quote!(unwrap_or_default()),
            ConcreteFallbackStrategy::Panic => {
                quote!(expect(concat!("field not set: ", #field_name_as_string)))
            }
        }
    }
}
