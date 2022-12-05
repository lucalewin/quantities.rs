use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};

pub(crate) struct DerivedAs {
    pub(crate) lhs_ident: syn::Ident,
    pub(crate) op: syn::BinOp,
    pub(crate) rhs_ident: syn::Ident,
}

pub(crate) struct UnitDef {
    pub(crate) unit_ident: syn::Ident,
    pub(crate) name: syn::LitStr,
    pub(crate) symbol: syn::LitStr,
    pub(crate) si_prefix: Option<syn::Ident>,
    pub(crate) scale: Option<syn::Lit>,
    pub(crate) doc: Option<syn::LitStr>,
}

pub(crate) struct QtyDef {
    pub(crate) qty_ident: syn::Ident,
    pub(crate) derived_by: Option<Derive>,
    pub(crate) ref_unit_ident: Option<syn::Ident>,
    pub(crate) units: Vec<UnitDef>,
}

impl QtyDef {
    fn new(qty_id: syn::Ident) -> Self {
        Self {
            qty_ident: qty_id,
            derived_by: None,
            ref_unit_ident: None,
            units: vec![],
        }
    }
}

pub(crate) type Item = syn::ItemStruct;

#[inline]
fn get_ident(expr: &syn::Expr) -> Option<&syn::Ident> {
    match expr {
        syn::Expr::Path(expr) => expr.path.get_ident(),
        _ => None,
    }
}

pub(crate) struct Derive {
    pub(crate) derives: Vec<DerivedAs>
}

impl syn::parse::Parse for Derive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        const ARGS_ERROR: &str =
            "Unknown argument(s) given to attribute `quantity`.";
        const OPERATOR_ERROR: &str = "Binary expression with '*' or '/' expected.";
        const OPERAND_ERROR: &str = "Identifier expected.";
        #[rustfmt::skip]
        const ARGS_HELP: &str =
            "Use `#[quantity]`\n\
            or  `#[quantity(<lhs_ident> * <rhs_ident>]`\n\
            or  `#[quantity(<lhs_ident> / <rhs_ident>]`.";

        let x = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input).unwrap();

        let mut derive = Derive { derives: Vec::with_capacity(x.len()) };

        for expr in x {
            match expr {
                syn::Expr::Binary(args) => match args.op {
                    syn::BinOp::Mul(_) | syn::BinOp::Div(_) => {
                        let lhs = get_ident(args.left.as_ref());
                        let rhs = get_ident(args.right.as_ref());
                        if lhs.is_none() || rhs.is_none() {
                            abort!(args, OPERAND_ERROR; help = ARGS_HELP)
                        }
                        derive.derives.push(DerivedAs {
                            lhs_ident: lhs.unwrap().clone(),
                            op: args.op,
                            rhs_ident: rhs.unwrap().clone(),
                        });
                    }
                    _ => abort!(args, OPERATOR_ERROR; help = ARGS_HELP),
                },
                _ => abort!(expr, ARGS_ERROR; help = ARGS_HELP),
            }
        }

        Ok(derive)
    }
}

pub(crate) fn parse_args(args: TokenStream) -> Option<Derive> {
    const ARGS_ERROR: &str =
        "Unknown argument(s) given to attribute `quantity`.";
    #[rustfmt::skip]
    const ARGS_HELP: &str =
        "Use `#[quantity]`\n\
         or  `#[quantity(<lhs_ident> * <rhs_ident>]`\n\
         or  `#[quantity(<lhs_ident> / <rhs_ident>]`.";

    if let Ok(derive) = syn::parse::<Derive>(args.into()) {
        Some(derive)
    } else {
        abort_call_site!(ARGS_ERROR; help = ARGS_HELP)
    }
}

pub(crate) fn parse_item(item: TokenStream) -> Item {
    #[rustfmt::skip]
    const ITEM_HELP: &str =
        "Use `#[quantity]\n\
              ...\n\
              struct <ident> {}`.";

    match syn::parse2::<Item>(item.clone()) {
        Ok(item) => item,
        Err(error) => abort!(item, error; help = ITEM_HELP),
    }
}

fn check_struct(ast: &Item) {
    const GENERICS_ERROR: &str =
        "Given struct must not have generic parameters.";
    const FIELDS_ERROR: &str = "Given struct must not have fields.";
    let help = format!("Use `struct {} {{}};`", ast.ident);

    if !ast.generics.params.is_empty() {
        abort!(ast.generics, GENERICS_ERROR; help = help.as_str());
    }
    if !ast.fields.is_empty() {
        abort!(ast.fields, FIELDS_ERROR; help = help.as_str());
    }
}

#[inline]
fn is_unit_attr(attr: &syn::Attribute) -> bool {
    attr.path
        .is_ident(&syn::Ident::new("unit", Span::call_site()))
}

#[inline]
fn is_ref_unit_attr(attr: &syn::Attribute) -> bool {
    attr.path
        .is_ident(&syn::Ident::new("ref_unit", Span::call_site()))
}

const ARGS_LIST_ERROR: &str =
    "A comma-separated list of 2 to 5 arguments expected.";

#[rustfmt::skip]
const UNIT_ATTR_HELP: &str =
    "Use `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>, \"<doc>\")]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>)]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <scale>, \"<doc>\")]`\n\
     or  `#[unit(<ident>, \"<symbol>\", <scale>)]`\n\
     or  `#[unit(<ident>, \"<symbol>\", \"<doc>\")]`\n\
     or  `#[unit(<ident>, \"<symbol>\")]`.";

fn get_unit_attrs(
    attrs: &Vec<syn::Attribute>,
) -> (Vec<syn::Attribute>, Option<syn::Attribute>) {
    const MORE_THAN_ONE_REFUNIT_ATTR_ERROR: &str =
        "There can only be one `refunit` attribute.";
    const NO_UNIT_ATTR_ERROR: &str =
        "At least one unit description must be given via attribute `unit`.";

    let mut unit_attrs: Vec<syn::Attribute> = vec![];
    let mut opt_ref_unit_attr: Option<syn::Attribute> = None;
    for attr in attrs {
        if is_unit_attr(attr) {
            unit_attrs.push(attr.clone());
        } else if is_ref_unit_attr(attr) {
            if opt_ref_unit_attr.is_some() {
                abort!(attr, MORE_THAN_ONE_REFUNIT_ATTR_ERROR);
            }
            opt_ref_unit_attr = Some(attr.clone());
        }
    }
    if unit_attrs.is_empty() {
        abort_call_site!(NO_UNIT_ATTR_ERROR; help = UNIT_ATTR_HELP);
    }
    (unit_attrs, opt_ref_unit_attr)
}

impl syn::parse::Parse for UnitDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut unit_ident: syn::Ident = input.parse()?;
        let _: syn::Token![,] = input.parse()?;
        let symbol: syn::LitStr = input.parse()?;
        let opt_comma: Option<syn::Token![,]> = input.parse()?;
        if opt_comma.is_none() && !input.is_empty() {
            return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
        }
        let mut si_prefix: Option<syn::Ident> = None;
        if input.peek(syn::Ident) {
            si_prefix = Some(input.parse::<syn::Ident>()?);
            let opt_comma: Option<syn::Token![,]> = input.parse()?;
            if opt_comma.is_none() && !input.is_empty() {
                return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
            }
        };
        let mut scale: Option<syn::Lit> = None;
        if input.peek(syn::LitFloat) || input.peek(syn::LitInt) {
            scale = Some(input.parse::<syn::Lit>()?);
            let opt_comma: Option<syn::Token![,]> = input.parse()?;
            if opt_comma.is_none() && !input.is_empty() {
                return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
            }
        };
        let mut doc: Option<syn::LitStr> = None;
        if input.peek(syn::LitStr) {
            doc = Some(input.parse::<syn::LitStr>()?);
        }
        // Check if input is exhausted:
        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), ARGS_LIST_ERROR));
        };
        let name = syn::LitStr::new(
            unit_ident.to_string().replace('_', " ").as_str(),
            Span::call_site(),
        );
        unit_ident = syn::Ident::new(
            unit_ident.to_string().to_case(Case::UpperCamel).as_str(),
            Span::call_site(),
        );
        Ok(UnitDef {
            unit_ident,
            name,
            symbol,
            si_prefix,
            scale,
            doc,
        })
    }
}

fn ref_unit_def_from_attr(ref_unit_attr: &syn::Attribute) -> UnitDef {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "2, 3 or 4 comma-separated args expected.";
    const WRONG_TYPE_OF_ARG_ERROR: &str = "No scale expected for ref_unit.";
    #[rustfmt::skip]
    const HELP: &str =
        "Use `#[ref_unit(<ident>, \"<symbol>\", <si_prefix>, \"<doc>\")]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\", <si_prefix>)]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\", \"<doc>\")]`\n\
         or  `#[ref_unit(<ident>, \"<symbol>\")]`.";

    match ref_unit_attr.parse_args::<UnitDef>() {
        Ok(mut unit_def) => {
            if unit_def.scale.is_some() {
                abort!(ref_unit_attr, WRONG_TYPE_OF_ARG_ERROR; help = HELP);
            }
            unit_def.scale = Some(syn::Lit::Float(syn::LitFloat::new(
                "1.0",
                Span::call_site(),
            )));
            unit_def
        }
        Err(_) => {
            abort!(ref_unit_attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
        }
    }
}

fn unit_defs_with_scale_from_attrs(
    attrs: &Vec<syn::Attribute>,
) -> Vec<UnitDef> {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "3, 4 or 5 comma-separated args expected.";
    const NO_SCALE_ERROR: &str = "<scale> arg expected.";
    #[rustfmt::skip]
    const HELP: &str =
        "Use `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>, \"<doc>\")]`
         or  `#[unit(<ident>, \"<symbol>\", <si_prefix>, <scale>)]`\n\
         or  `#[unit(<ident>, \"<symbol>\", <scale>, \"<doc>\")]`\n\
         or  `#[unit(<ident>, \"<symbol>\", <scale>)]`.";

    let mut unit_defs: Vec<UnitDef> = vec![];
    for attr in attrs {
        match attr.parse_args::<UnitDef>() {
            Ok(unit_def) => {
                if unit_def.scale.is_none() {
                    abort!(attr, NO_SCALE_ERROR; help = HELP);
                }
                unit_defs.push(unit_def);
            }
            Err(_) => {
                abort!(attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
            }
        }
    }
    unit_defs
}

fn unit_defs_without_scale_from_attrs(
    attrs: &Vec<syn::Attribute>,
) -> Vec<UnitDef> {
    const WRONG_NUMBER_OF_ARGS_ERROR: &str =
        "2 or 3 comma-separated args expected.";
    #[rustfmt::skip]
    const HELP: &str =
        "Use `#[unit(<ident>, \"<symbol>\", \"<doc>\")]`\n\
         or  `#[unit(<ident>, \"<symbol>\")]`.";

    let mut unit_defs: Vec<UnitDef> = vec![];
    for attr in attrs {
        match attr.parse_args::<UnitDef>() {
            Ok(unit_def) => {
                if unit_def.scale.is_some() || unit_def.si_prefix.is_some() {
                    abort!(attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
                }
                unit_defs.push(unit_def);
            }
            Err(_) => {
                abort!(attr, WRONG_NUMBER_OF_ARGS_ERROR; help = HELP);
            }
        }
    }
    unit_defs
}

#[inline]
pub(crate) fn opt_lit_to_f64(lit: &Option<syn::Lit>) -> f64 {
    match lit.as_ref().unwrap() {
        syn::Lit::Float(f) => f.base10_parse().unwrap(),
        syn::Lit::Int(i) => i.base10_parse().unwrap(),
        _ => abort!(lit, "Internal error: unexspected non-numeric literal."),
    }
}

pub(crate) fn analyze(item_ast: &mut Item) -> QtyDef {
    check_struct(item_ast);
    let attrs = &mut item_ast.attrs;
    let (unit_attrs, opt_ref_unit_attr) = get_unit_attrs(attrs);
    attrs.retain(|attr| !(is_unit_attr(attr) || is_ref_unit_attr(attr)));
    let mut qty_def = QtyDef::new(item_ast.ident.clone());
    if let Some(ref_unit_attr) = opt_ref_unit_attr {
        let ref_unit_def = ref_unit_def_from_attr(&ref_unit_attr);
        qty_def.ref_unit_ident = Some(ref_unit_def.unit_ident.clone());
        qty_def.units = unit_defs_with_scale_from_attrs(&unit_attrs);
        qty_def.units.insert(0, ref_unit_def);
        qty_def.units.sort_by(|a, b| {
            let x = opt_lit_to_f64(&a.scale);
            let y = opt_lit_to_f64(&b.scale);
            x.partial_cmp(&y).unwrap()
        });
    } else {
        qty_def.units = unit_defs_without_scale_from_attrs(&unit_attrs);
        qty_def
            .units
            .sort_by(|a, b| a.name.value().cmp(&b.name.value()));
    }
    qty_def
}