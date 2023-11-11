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
    let orig_visibility = &input.vis;

    let new_enum_name = syn::Ident::new(&format!("{}Light", orig_enum_name), orig_enum_name.span());

    let light_variants = data_enum.variants.iter().map(|variant| &variant.ident);
    let light_variants_cloned = light_variants.clone();

    let generated_code = quote! {
        use light_enum::Values;

        #[derive(Debug, PartialEq, Eq, Clone, Values)]
        #orig_visibility enum #new_enum_name {
            #(
                #light_variants,
            )*
        }

        impl #orig_enum_name {
            #orig_visibility fn to_light(&self) -> #new_enum_name {
                match self {
                    #(
                        #orig_enum_name::#light_variants_cloned(_) => #new_enum_name::#light_variants_cloned,
                    )*
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

    let orig_enum_name = &input.ident;
    let orig_visibility = &input.vis;

    let variants = data_enum.variants.iter().map(|variant| &variant.ident);

    let variants_count = variants.clone().count();

    let generated_code = quote! {

        impl #orig_enum_name {

            #orig_visibility const VALUES: [#orig_enum_name; #variants_count] = [
                #(
                     #orig_enum_name::#variants,
                )*
            ];

        }
    };

    generated_code.into()
}
