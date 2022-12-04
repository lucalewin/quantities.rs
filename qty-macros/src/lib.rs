// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

#![doc = include_str ! ("../README.md")]

mod parsing;
mod codegen;

use ::convert_case::{Case, Casing};
use ::proc_macro::TokenStream;
use ::proc_macro2::{Span, TokenStream as TokenStream2};
use ::proc_macro_error::proc_macro_error;
use ::quote::quote;
use ::syn::{parse_macro_input, Ident, ItemEnum};

use crate::parsing::{analyze, parse_args, parse_item};
use crate::codegen::codegen;

/// Derives a constant for each variant of a fieldless enum.
///
/// # Panics
///
/// The macro panics in the following cases:
///
/// * The attributed item is not an enum.
/// * The enum is not a fieldless enum (aka C-like enum).
///
/// # Example
///
/// ```rust
/// # use qty_macros::VariantsAsConstants;
/// # #[allow(non_camel_case_types)]
/// #[derive(VariantsAsConstants)]
/// enum TestEnum {
///     MultiCamelCase,
///     snake_case,
///     simple,
///     ALL_UPPER,
/// }
/// ```
///
/// This results in the following additional code:
///
/// ```rust
/// # #[allow(non_camel_case_types)]
/// # enum TestEnum {
/// #     MultiCamelCase,
/// #     snake_case,
/// #     simple,
/// #     ALL_UPPER,
/// # }
/// pub const MULTI_CAMEL_CASE: TestEnum = TestEnum::MultiCamelCase;
/// pub const SNAKE_CASE: TestEnum = TestEnum::snake_case;
/// pub const SIMPLE: TestEnum = TestEnum::simple;
/// pub const ALL_UPPER: TestEnum = TestEnum::ALL_UPPER;
/// ```
#[proc_macro_derive(VariantsAsConstants)]
pub fn derive_variants_as_constants(input: TokenStream) -> TokenStream {
    let enum_def = parse_macro_input!(input as ItemEnum);
    let mut output = TokenStream2::new();
    let enum_ident = enum_def.ident;
    // check and gather const declarations for variants
    for variant in enum_def.variants {
        if !variant.fields.is_empty() {
            panic!("The given enum must be a fieldless enum.");
        }
        let variant_ident = variant.ident;
        let const_ident = Ident::new(
            variant_ident.to_string().to_case(Case::UpperSnake).as_str(),
            Span::call_site(),
        );
        output = quote!(
            #output
            pub const #const_ident: #enum_ident = #enum_ident::#variant_ident;
        );
    }
    output.into()
}

/// Derives a function `iter` that returns an iterator over the variants of a
/// fieldless enum.
///
/// # Panics
///
/// The macro panics in the following cases:
///
/// * The attributed item is not an enum.
/// * The enum is not a fieldless enum (aka C-like enum).
///
/// # Example
///
/// ```rust
/// # use qty_macros::VariantsAsConstants;
/// #[derive(VariantsAsConstants)]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
/// ```
///
/// This results in the following additional code:
///
/// ```rust
/// # enum Color {
/// #     Red,
/// #     Green,
/// #     Blue,
/// # }
/// impl Color {
///     const VARIANTS: [Self; 3usize] =
///         [Self::Red, Self::Green, Self::Blue];
///     #[doc = "Returns an iterator over the variants of `Self`."]
///     #[inline(always)]
///     pub fn iter() -> core::slice::Iter<'static, Self> {
///         Self::VARIANTS.iter()
///     }
/// }
/// ```
#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(input: TokenStream) -> TokenStream {
    let enum_def = parse_macro_input!(input as ItemEnum);
    let enum_ident = enum_def.ident;
    let mut output = TokenStream2::new();
    // check and gather variants
    for variant in &enum_def.variants {
        if !variant.fields.is_empty() {
            panic!("The given enum must be a fieldless enum.");
        }
        let variant_ident = &variant.ident;
        output = quote!(
            #output
            Self::#variant_ident,
        );
    }
    let n_variants = &enum_def.variants.len();
    // create impl for fn iter
    output = quote!(
        impl #enum_ident {
            const VARIANTS: [Self; #n_variants] = [#output];
            #[doc = "Returns an iterator over the variants of `Self`."]
            #[inline(always)]
            pub fn iter() -> core::slice::Iter<'static, Self> {
                Self::VARIANTS.iter()
            }
        }
    );
    output.into()
}

/// Generates an enum with the given units (incl. the refunit, if given) as
/// variants, an implemention of trait `Unit` for this enum and a type alias of
/// `Qty` with the enum as parameter and named after the given struct.
///
/// In addition, it creates a constant for each enum variant, thus providing a
/// constant for each unit.
///
/// This implies that the identifiers of all units over all defined
/// quantitities have to be unique!
///
/// The attribute `#[quantity]` can optionally be followed by an attribute
/// `#[ref_unit]` and must be followed by at least one attribute `#[unit]`.
///
/// To define a quantity with a reference unit, use one of the following forms
/// of the ref_unit attribute
///
/// `#[ref_unit(<ident>, "<symbol>", <si_prefix>, "<doc>")]`
/// `#[ref_unit(<ident>, "<symbol>", <si_prefix>)]`
/// `#[ref_unit(<ident>, "<symbol>", "<doc>")]`,
/// `#[ref_unit(<ident>, "<symbol>")]`,
///
/// followed by one ore more unit attributes in one of the following forms
///
/// `#[unit(<ident>, "<symbol>", <si_prefix>, <scale>, "<doc>")]`
/// `#[unit(<ident>, "<symbol>", <si_prefix>, <scale>)]`
/// `#[unit(<ident>, "<symbol>", <scale>, "<doc>")]`.
/// `#[unit(<ident>, "<symbol>", <scale>)]`.
///
/// To define a quantity without a reference unit, use one ore more unit
/// attributes in one of the following forms
///
/// `#[unit(<ident>, "<symbol>", "<doc>")]`
/// `#[unit(<ident>, "<symbol>")]`.
///
/// # Panics
///
/// The macro panics in the followong cases:
///
/// * Invalid arguments given to the attribute `#[quantity]`.
/// * The given item is not a struct.
/// * The given struct does have generic parameters and/or fields.
/// * More than one attribute `#[ref_unit]` is given.
/// * No attribute `#[unit]` is given.
/// * Wrong number or wrong type of arguments given to attribute `#[ref_unit]`.
/// * Wrong number of arguments given to an attribute `#[unit]`.
/// * No \<scale\> argument given to an attribute `#[unit]` when required.
#[proc_macro_attribute]
#[proc_macro_error]
pub fn quantity(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_ast = parse_item(item.into());
    let mut qty_def = analyze(&mut item_ast);
    qty_def.derived_as = parse_args(args.into());
    let code = codegen(&qty_def, &item_ast.attrs);
    code.into()
}
