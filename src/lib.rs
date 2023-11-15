//!  A crate providing a derive keyword to generate a light enum.
//!
//! This crate provide two derive keywords:
//! - `LightEnum` will generate a new enum without the content of each field
//! - `Values` will generate a vector containing each field of the enum

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Generate a new enum `[YourEnumName]Light` without the content of each field
///
/// ## Example
///
/// ```
/// use light_enum::LightEnum;
///
/// #[derive(LightEnum)]
/// enum MyEnum {
///     A(i32, i32),
///     B(i32),
///     C,
/// }
///
/// let heavy = MyEnum::A(0, 0);
/// let light = heavy.to_light();
/// assert!(light == MyEnumLight::A);
/// ```
///
/// ## Generated code
///
/// ```ignore
/// enum MyEnumLight {
///     A,
///     B,
///     C,
/// }
/// ```
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
            /// Get the equivalent field of the [`Light`] enum
            #visibility fn to_light(&self) -> #new_enum_name {
                match self {
                    #(#to_light_match_lines),*
                }
            }
        }
    };

    generated_code.into()
}

/// Generate an array containing each field of an enum.
///
/// ## Example
///
/// ```
/// use light_enum::Values;
///
/// #[derive(Values, PartialEq, Eq)]
/// enum Vals {
///     A,
///     B,
///     C,
/// }
///
/// let values = Vals::VALUES;
/// assert!(values.len() == 3);
/// assert!(values.contains(&Vals::A));
/// assert!(values.contains(&Vals::B));
/// assert!(values.contains(&Vals::C));
/// ```
///
/// ## Generated code
///
/// ```ignore
/// const VALUES: [Vals; 3usize] = [Vals::A, Vals::B, Vals::C];
/// ```
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
            /// Static array containing each field of your enum.
            #visibility const VALUES: [#enum_name; #fields_count] = [
                #(
                     #enum_name::#fields
                ),*
            ];

        }
    };

    generated_code.into()
}
