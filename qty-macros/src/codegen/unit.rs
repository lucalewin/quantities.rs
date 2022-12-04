use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::parsing::*;

pub fn codegen_attrs(attrs: &Vec<syn::Attribute>) -> TokenStream {
    let mut code = TokenStream::new();
    for attr in attrs {
        code = quote!(
            #code
            #attr
        );
    }
    code
}

pub(crate) fn codegen_unit_constants(
    enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let const_ident = syn::Ident::new(
            unit_ident.to_string().to_case(Case::UpperSnake).as_str(),
            Span::call_site(),
        );
        match &unit.doc {
            None => {
                code = quote!(
                    #code
                    pub const #const_ident: #enum_ident =
                        #enum_ident::#unit_ident;
                )
            }
            Some(doc) => {
                let unit_doc = doc.value();
                code = quote!(
                    #code
                    #[doc = #unit_doc]
                    pub const #const_ident: #enum_ident =
                        #enum_ident::#unit_ident;
                )
            }
        };
    }
    code
}

pub(crate) fn codegen_fn_si_prefix(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        if unit.si_prefix.is_some() {
            let unit_ident = &unit.unit_ident;
            let unit_si_prefix: &syn::Ident = unit.si_prefix.as_ref().unwrap();
            code = quote!(
                #code
                Self::#unit_ident =>
                    Some(SIPrefix::#unit_si_prefix),
            )
        }
    }
    quote!(
        fn si_prefix(&self) -> Option<SIPrefix> {
            match self {
                #code
                _ => None,
            }
        }
    )
}

pub(crate) fn codegen_fn_scale(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        if unit.scale.is_some() {
            let unit_ident = &unit.unit_ident;
            let unit_scale: &syn::Lit = unit.scale.as_ref().unwrap();
            code = quote!(
                #code
                Self::#unit_ident => #unit_scale as Amount,
            )
        } else {
            // should not happen!
            abort_call_site!("Missing scale detected!")
        }
    }
    quote!(
        fn scale(&self) -> Amount {
            match self {
                #code
            }
        }
    )
}

pub(crate) fn codegen_impl_mul_amnt_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl const Mul<#unit_enum_ident> for Amount {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: #unit_enum_ident) -> Self::Output {
                Self::Output::new(self, rhs)
            }
        }
        impl const Mul<Amount> for #unit_enum_ident {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: Amount) -> Self::Output {
                Self::Output::new(rhs, self)
            }
        }
    )
}

pub fn codegen_qty_single_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
    unit_ident: &syn::Ident,
    unit_name: &syn::LitStr,
    unit_symbol: &syn::LitStr,
) -> TokenStream {
    let unit_doc = format!("Unit of quantity `{}`.", qty_ident);
    quote!(
        #[doc = #unit_doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum #unit_enum_ident {
            #unit_ident,
        }
        impl #unit_enum_ident {
            const VARIANTS: [Self; 1] = [Self::#unit_ident];
        }
        impl Unit for #unit_enum_ident {
            type QuantityType = #qty_ident;
            fn iter<'a>() -> core::slice::Iter<'a, Self> {
                Self::VARIANTS.iter()
            }
            fn name(&self) -> &'static str { #unit_name }
            fn symbol(&self) -> &'static str { #unit_symbol }
            fn si_prefix(&self) -> Option<SIPrefix> { None }
        }
        #[derive(Copy, Clone, Debug)]
        pub struct #qty_ident {
            value: Amount
        }
        impl Quantity for #qty_ident {
            type UnitType = #unit_enum_ident;

            #[inline(always)]
            fn new(value: Amount, _unit: Self::UnitType) -> Self {
                Self { value }
            }

            #[inline(always)]
            fn value(&self) -> Amount {
                self.value
            }

            #[inline(always)]
            fn unit(&self) -> Self::UnitType {
                Self::UnitType::#unit_ident
            }
        }
        impl Add<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.value() + rhs.value(), self.unit())
            }
        }
        impl Sub<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.value() - rhs.value(), self.unit())
            }
        }
        impl Div<Self> for #qty_ident {
            type Output = Amount;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                self.value() / rhs.value()
            }
        }
    )
}

pub(crate) fn codegen_unit_variants(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        match &unit.doc {
            None => {
                code = quote!(
                    #code
                    #unit_ident,
                )
            }
            Some(doc) => {
                let unit_doc = doc.value();
                code = quote!(
                    #code
                    #[doc = #unit_doc]
                    #unit_ident,
                )
            }
        };
    }
    code
}

pub(crate) fn codegen_unit_variants_array(
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let mut code = TokenStream::new();
    let n_variants = units.len();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        code = quote!(
            #code
            Self::#unit_ident,
        );
    }
    code = quote!(
        impl #unit_enum_ident {
            const VARIANTS: [Self; #n_variants] = [#code];
        }
    );
    code
}

pub(crate) fn codegen_fn_name(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let unit_name = unit.name.clone();
        code = quote!(
            #code
            Self::#unit_ident => #unit_name,
        )
    }
    quote!(
        fn name(&self) -> &'static str {
            match self {
                #code
            }
        }
    )
}

pub(crate) fn codegen_fn_symbol(units: &Vec<UnitDef>) -> TokenStream {
    let mut code = TokenStream::new();
    for unit in units {
        let unit_ident = unit.unit_ident.clone();
        let unit_symbol = unit.symbol.clone();
        code = quote!(
            #code
            Self::#unit_ident => #unit_symbol,
        )
    }
    quote!(
        fn symbol(&self) -> &'static str {
            match self {
                #code
            }
        }
    )
}

pub(crate) fn codegen_impl_unit_display(unit_enum_ident: &syn::Ident) -> TokenStream {
    quote!(
        impl fmt::Display for #unit_enum_ident {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <Self as Unit>::fmt(self, f)
            }
        }
    )
}
