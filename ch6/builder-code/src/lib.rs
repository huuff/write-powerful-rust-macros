mod fields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

use fields::{
    builder_field_definitions, builder_field_init, builder_methods, original_struct_setters,
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();

    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for structs with named fields"),
    };

    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_field_init(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields);

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_struct_name_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct Struct {
                field: String
            }
        };
        let expected_builder = quote! {
            struct StructBuilder {
                field: Option<String>,
            }
        };

        let expected_builder_methods = quote! {
            impl StructBuilder {
                pub fn field(mut self, input: String) -> Self {
                    self.field = Some(input);
                    self
                }

                pub fn build(self) -> Struct {
                    Struct {
                        field: self.field.expect(concat!("field not set: ", "field")),
                    }
                }
            }
        };

        let actual = create_builder(input);

        println!("actual: {actual}");
        println!("expected builder methods: {expected_builder_methods}");
        assert!(actual.to_string().contains(&expected_builder.to_string()));
        assert!(actual
            .to_string()
            .contains(&expected_builder_methods.to_string()));
    }

    #[test]
    #[ignore]
    fn assert_with_parsing() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        let derived: DeriveInput = syn::parse2(actual).unwrap();
        let name = derived.ident;

        assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    }
}
