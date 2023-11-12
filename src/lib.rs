extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(LightEnum)]
pub fn generate_light_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data_enum) = &input.data else {
        panic!("LightEnum can only be derived for enums");
    };

    let orig_enum_name = &input.ident;
    let visibility = &input.vis;

    let new_enum_name = syn::Ident::new(&format!("{}Light", orig_enum_name), orig_enum_name.span());

    let light_fields = data_enum.variants.iter().map(|variant| &variant.ident);

    let to_light_match_lines = data_enum.variants.iter().map(|variant| {
        let field = &variant.ident;
        if variant.fields.is_empty() {
            quote! { #orig_enum_name::#field => #new_enum_name::#field }
        } else {
            quote! { #orig_enum_name::#field(..) => #new_enum_name::#field }
        }
    });

    let generated_code = quote! {
        use light_enum::Values;

        #[derive(Debug, PartialEq, Eq, Clone, Values)]
        #visibility enum #new_enum_name {
            #(
                #light_fields
            ),*
        }

        impl #orig_enum_name {
            #visibility fn to_light(&self) -> #new_enum_name {
                match self {
                    #(#to_light_match_lines),*
                }
            }
        }
    };

    generated_code.into()
}

#[proc_macro_derive(Values)]
pub fn generate_values(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(data_enum) = &input.data else {
        panic!("Values can only be derived for enums");
    };

    let enum_name = &input.ident;
    let visibility = &input.vis;

    let fields = data_enum.variants.iter().map(|variant| &variant.ident);

    let fields_count = fields.clone().count();

    let generated_code = quote! {

        impl #enum_name {

            #visibility const VALUES: [#enum_name; #fields_count] = [
                #(
                     #enum_name::#fields
                ),*
            ];

        }
    };

    generated_code.into()
}
