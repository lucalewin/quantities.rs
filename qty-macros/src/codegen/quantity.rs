use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::parsing::*;
use crate::codegen::unit::*;

pub fn codegen_impl_quantity(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        #[derive(Copy, Clone, Debug)]
        pub struct #qty_ident {
            value: Amount,
            unit: #unit_enum_ident
        }
        impl const Quantity for #qty_ident {
            type UnitType = #unit_enum_ident;
            #[inline(always)]
            fn new(value: Amount, unit: Self::UnitType) -> Self {
                Self { value, unit }
            }
            #[inline(always)]
            fn value(&self) -> Amount {
                self.value
            }
            #[inline(always)]
            fn unit(&self) -> Self::UnitType {
                self.unit
            }
        }
        impl QuantityImpl for #qty_ident {}
    )
}

pub(crate) fn codegen_qty_with_ref_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
    ref_unit_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let code_unit_variants = codegen_unit_variants(units);
    let code_unit_variants_array =
        codegen_unit_variants_array(unit_enum_ident, units);
    let code_fn_name = codegen_fn_name(units);
    let code_fn_symbol = codegen_fn_symbol(units);
    let code_fn_si_prefix = codegen_fn_si_prefix(units);
    let code_fn_scale = codegen_fn_scale(units);
    let unit_doc = format!("Unit of quantity `{}`.", qty_ident);
    let code_impl_quantity = codegen_impl_quantity(qty_ident, unit_enum_ident);
    quote!(
        #code_impl_quantity
        #[doc = #unit_doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum #unit_enum_ident {
            #code_unit_variants
        }
        #code_unit_variants_array
        impl Unit for #unit_enum_ident {
            type QuantityType = #qty_ident;
            fn iter<'a>() -> core::slice::Iter<'a, Self> {
                Self::VARIANTS.iter()
            }
            #code_fn_name
            #code_fn_symbol
            #code_fn_si_prefix
        }
        impl LinearScaledUnit for #unit_enum_ident {
            const REF_UNIT: Self = Self::#ref_unit_ident;
            #code_fn_scale
        }
        impl HasRefUnit for #qty_ident {
            const REF_UNIT: #unit_enum_ident =
                #unit_enum_ident::#ref_unit_ident;
        }
        impl Eq for #qty_ident {}
        impl PartialEq<Self> for #qty_ident {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                <Self as HasRefUnit>::eq(self, other)
            }
        }
        impl PartialOrd for #qty_ident {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                <Self as HasRefUnit>::partial_cmp(self, other)
            }
        }
        impl Add<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                <Self as HasRefUnit>::add(self, rhs)
            }
        }
        impl Sub<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                <Self as HasRefUnit>::sub(self, rhs)
            }
        }
        impl Div<Self> for #qty_ident {
            type Output = Amount;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                <Self as HasRefUnit>::div(self, rhs)
            }
        }
    )
}

pub(crate) fn codegen_qty_without_ref_unit(
    qty_ident: &syn::Ident,
    unit_enum_ident: &syn::Ident,
    units: &Vec<UnitDef>,
) -> TokenStream {
    let code_unit_variants = codegen_unit_variants(units);
    let code_unit_variants_array =
        codegen_unit_variants_array(unit_enum_ident, units);
    let code_fn_name = codegen_fn_name(units);
    let code_fn_symbol = codegen_fn_symbol(units);
    let unit_doc = format!("Unit of quantity `{}`.", qty_ident);
    let code_impl_quantity = codegen_impl_quantity(qty_ident, unit_enum_ident);
    quote!(
        #code_impl_quantity
        #[doc = #unit_doc]
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum #unit_enum_ident { #code_unit_variants }
        #code_unit_variants_array
        impl Unit for #unit_enum_ident {
            type QuantityType = #qty_ident;
            fn iter<'a>() -> core::slice::Iter<'a, Self> {
                Self::VARIANTS.iter()
            }
            #code_fn_name
            #code_fn_symbol
            fn si_prefix(&self) -> Option<SIPrefix> { None }
        }
        impl Eq for #qty_ident {}
        impl PartialEq<Self> for #qty_ident {
            #[inline(always)]
            fn eq(&self, other: &Self) -> bool {
                <Self as QuantityImpl>::eq(self, other)
            }
        }
        impl PartialOrd for #qty_ident {
            #[inline(always)]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                <Self as QuantityImpl>::partial_cmp(self, other)
            }
        }
        impl Add<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                <Self as QuantityImpl>::add(self, rhs)
            }
        }
        impl Sub<Self> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self::Output {
                <Self as QuantityImpl>::sub(self, rhs)
            }
        }
        impl Div<Self> for #qty_ident {
            type Output = Amount;
            #[inline(always)]
            fn div(self, rhs: Self) -> Self::Output {
                <Self as QuantityImpl>::div(self, rhs)
            }
        }
    )
}

pub fn codegen_impl_std_traits(qty_ident: &syn::Ident) -> TokenStream {
    quote!(
        impl fmt::Display for #qty_ident {
            #[inline(always)]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <Self as QuantityImpl>::fmt(self, f)
            }
        }
        impl Mul<#qty_ident> for Amount {
            type Output = #qty_ident;
            #[inline(always)]
            fn mul(self, rhs: #qty_ident) -> Self::Output {
                Self::Output::new(self * rhs.value(), rhs.unit())
            }
        }
        impl Mul<Amount> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: Amount) -> Self::Output {
                Self::Output::new(self.value() * rhs, self.unit())
            }
        }
        impl Div<Amount> for #qty_ident {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: Amount) -> Self::Output {
                Self::Output::new(self.value() / rhs, self.unit())
            }
        }
        impl<TQ: Quantity> Mul<Rate<TQ, Self>> for #qty_ident {
            type Output = TQ;

            fn mul(self, rhs: Rate<TQ, Self>) -> Self::Output {
                let amnt: Amount =
                    (self / rhs.per_unit().as_qty()) / rhs.per_unit_multiple();
                Self::Output::new(amnt * rhs.term_amount(), rhs.term_unit())
            }
        }
        impl<PQ: Quantity> Div<Rate<Self, PQ>> for #qty_ident {
            type Output = PQ;

            fn div(self, rhs: Rate<Self, PQ>) -> Self::Output {
                let amnt: Amount =
                    (self / rhs.term_unit().as_qty()) / rhs.term_amount();
                Self::Output::new(
                    amnt * rhs.per_unit_multiple(),
                    rhs.per_unit()
                )
            }
        }
    )
}

pub fn codegen_impl_qty_sqared(
    res_qty_ident: &syn::Ident,
    qty_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Mul<Self> for #qty_ident
        where
            Self: HasRefUnit,
        {
            type Output = #res_qty_ident;
            fn mul(self, rhs: Self) -> Self::Output {
                let scale =
                    self.unit().scale() * rhs.unit().scale();
                match Self::Output::unit_from_scale(scale) {
                    Some(unit) =>
                        Self::Output::new(self.value() * rhs.value(), unit),
                    None =>
                        <Self::Output as HasRefUnit>::_fit(
                            self.value() * rhs.value() * scale
                        )
                }
            }
        }
        impl<'a> Mul<#qty_ident> for &'a #qty_ident
        where
            #qty_ident: Mul<#qty_ident>,
        {
            type Output = <#qty_ident as Mul<#qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: #qty_ident) -> Self::Output {
                Mul::mul(*self, rhs)
            }
        }
        impl Mul<&Self> for #qty_ident
        where
            Self: Mul<Self>,
        {
            type Output = <Self as Mul<Self>>::Output;
            #[inline(always)]
            fn mul(self, rhs: &Self) -> Self::Output {
                Mul::mul(self, *rhs)
            }
        }
        impl Mul<Self> for &#qty_ident
        where
            #qty_ident: Mul<#qty_ident>,
        {
            type Output = <#qty_ident as Mul<#qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: Self) -> Self::Output {
                Mul::mul(*self, *rhs)
            }
        }
    )
}

pub fn codegen_impl_qty_mul_qty(
    res_qty_ident: &syn::Ident,
    lhs_qty_ident: &syn::Ident,
    rhs_qty_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Mul<#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: HasRefUnit,
            #rhs_qty_ident: HasRefUnit,
        {
            type Output = #res_qty_ident;
            fn mul(self, rhs: #rhs_qty_ident) -> Self::Output {
                let scale =
                    self.unit().scale() * rhs.unit().scale();
                match Self::Output::unit_from_scale(scale) {
                    Some(unit) =>
                        Self::Output::new(self.value() * rhs.value(), unit),
                    None =>
                        <Self::Output as HasRefUnit>::_fit(
                            self.value() * rhs.value() * scale
                        )
                }
            }
        }
        impl<'a> Mul<#rhs_qty_ident> for &'a #lhs_qty_ident
        where
            #lhs_qty_ident: Mul<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Mul<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: #rhs_qty_ident) -> Self::Output {
                Mul::mul(*self, rhs)
            }
        }
        impl Mul<&#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: Mul<#rhs_qty_ident>,
        {
            type Output = <Self as Mul<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Mul::mul(self, *rhs)
            }
        }
        impl Mul<&#rhs_qty_ident> for &#lhs_qty_ident
        where
            #lhs_qty_ident: Mul<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Mul<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn mul(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Mul::mul(*self, *rhs)
            }
        }
    )
}

pub fn codegen_impl_mul_qties(
    res_qty_ident: &syn::Ident,
    lhs_qty_ident: &syn::Ident,
    rhs_qty_ident: &syn::Ident,
) -> TokenStream {
    if lhs_qty_ident == rhs_qty_ident {
        let code = codegen_impl_qty_sqared(res_qty_ident, lhs_qty_ident);
        quote!(
            #code
        )
    } else {
        let code_lr = codegen_impl_qty_mul_qty(
            res_qty_ident,
            lhs_qty_ident,
            rhs_qty_ident,
        );
        let code_rl = codegen_impl_qty_mul_qty(
            res_qty_ident,
            rhs_qty_ident,
            lhs_qty_ident,
        );
        quote!(
            #code_lr
            #code_rl
        )
    }
}

pub fn codegen_impl_div_qties(
    res_qty_ident: &syn::Ident,
    lhs_qty_ident: &syn::Ident,
    rhs_qty_ident: &syn::Ident,
) -> TokenStream {
    quote!(
        impl Div<#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: HasRefUnit,
            #rhs_qty_ident: HasRefUnit,
        {
            type Output = #res_qty_ident;
            fn div(self, rhs: #rhs_qty_ident) -> Self::Output {
                let scale =
                    self.unit().scale() / rhs.unit().scale();
                match Self::Output::unit_from_scale(scale) {
                    Some(unit) =>
                        Self::Output::new(self.value() / rhs.value(), unit),
                    None =>
                        <Self::Output as HasRefUnit>::_fit(
                            (self.value() / rhs.value()) * scale
                        )
                }
            }
        }
        impl<'a> Div<#rhs_qty_ident> for &'a #lhs_qty_ident
        where
            #lhs_qty_ident: Div<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Div<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn div(self, rhs: #rhs_qty_ident) -> Self::Output {
                Div::div(*self, rhs)
            }
        }
        impl Div<&#rhs_qty_ident> for #lhs_qty_ident
        where
            Self: Div<#rhs_qty_ident>,
        {
            type Output = <Self as Div<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn div(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Div::div(self, *rhs)
            }
        }
        impl Div<&#rhs_qty_ident> for &#lhs_qty_ident
        where
            #lhs_qty_ident: Div<#rhs_qty_ident>,
        {
            type Output = <#lhs_qty_ident as Div<#rhs_qty_ident>>::Output;
            #[inline(always)]
            fn div(self, rhs: &#rhs_qty_ident) -> Self::Output {
                Div::div(*self, *rhs)
            }
        }
    )
}

pub(crate) fn codegen_impl_mul_div_qties(
    qty_ident: &syn::Ident,
    derived_as: &Option<DerivedAs>,
) -> TokenStream {
    match derived_as {
        None => TokenStream::new(),
        Some(derived_as) => {
            let lhs_qty_ident = &derived_as.lhs_ident;
            let rhs_qty_ident = &derived_as.rhs_ident;
            match derived_as.op {
                syn::BinOp::Mul(_) => {
                    let code_impl_mul = codegen_impl_mul_qties(
                        qty_ident,
                        lhs_qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_res_div_rhs = codegen_impl_div_qties(
                        lhs_qty_ident,
                        qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_res_div_lhs =
                        if lhs_qty_ident == rhs_qty_ident {
                            TokenStream::new()
                        } else {
                            codegen_impl_div_qties(
                                rhs_qty_ident,
                                qty_ident,
                                lhs_qty_ident,
                            )
                        };
                    quote!(
                        #code_impl_mul
                        #code_impl_res_div_rhs
                        #code_impl_res_div_lhs
                    )
                }
                syn::BinOp::Div(_) => {
                    let code_impl_div = codegen_impl_div_qties(
                        qty_ident,
                        lhs_qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_mul_res = codegen_impl_mul_qties(
                        lhs_qty_ident,
                        qty_ident,
                        rhs_qty_ident,
                    );
                    let code_impl_div_res = codegen_impl_div_qties(
                        rhs_qty_ident,
                        lhs_qty_ident,
                        qty_ident,
                    );
                    quote!(
                        #code_impl_div
                        #code_impl_mul_res
                        #code_impl_div_res
                    )
                }
                _ => {
                    // should not happen!
                    abort_call_site!("Internal error: wrong op detected!")
                }
            }
        }
    }
}
