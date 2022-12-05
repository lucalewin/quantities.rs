pub mod unit;
pub mod quantity;

use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::parsing::*;
use crate::codegen::unit::*;
use crate::codegen::quantity::*;

pub(crate) fn codegen(
    qty_def: &QtyDef,
    attrs: &Vec<syn::Attribute>,
) -> TokenStream {
    let qty_ident = qty_def.qty_ident.clone();
    let unit_enum_ident =
        syn::Ident::new(&format!("{}Unit", qty_ident), Span::call_site());
    let code_attrs = codegen_attrs(attrs);
    let code_qty = if qty_def.units.len() == 1 {
        let unit_ident = qty_def.units[0].unit_ident.clone();
        let unit_name = qty_def.units[0].name.clone();
        let unit_symbol = qty_def.units[0].symbol.clone();
        codegen_qty_single_unit(
            &qty_ident,
            &unit_enum_ident,
            &unit_ident,
            &unit_name,
            &unit_symbol,
        )
    } else if qty_def.ref_unit_ident.is_none() {
        codegen_qty_without_ref_unit(
            &qty_ident,
            &unit_enum_ident,
            &qty_def.units,
        )
    } else {
        let ref_unit_ident: &syn::Ident =
            qty_def.ref_unit_ident.as_ref().unwrap();
        codegen_qty_with_ref_unit(
            &qty_ident,
            &unit_enum_ident,
            ref_unit_ident,
            &qty_def.units,
        )
    };
    let code_unit_consts =
        codegen_unit_constants(&unit_enum_ident, &qty_def.units);
    let code_impl_mul =
        codegen_impl_mul_amnt_unit(&qty_ident, &unit_enum_ident);
    let code_impl_unit_display = codegen_impl_unit_display(&unit_enum_ident);
    let code_impl_std_traits = codegen_impl_std_traits(&qty_ident);
    let code_mul_div_base_qties = {
        let mut code = quote::quote!();

        match &qty_def.derived_by {
            Some(derive) => {
                for derived_as in &derive.derives {
                    let d_a = codegen_impl_mul_div_qties(&qty_ident, Some(derived_as));
                    code = quote::quote!(
                        #code
                        #d_a
                    )
                }
            },
            None => todo!("codegen(): None branch not implemented yet")
        }

        code
    };
    quote!(
        #code_attrs
        #code_qty
        #code_unit_consts
        #code_impl_mul
        #code_impl_unit_display
        #code_impl_std_traits
        #code_mul_div_base_qties
    )
}
