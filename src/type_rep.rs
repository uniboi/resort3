use sqparse::ast::{Type, TypeDefinitionStatement};

use crate::get_expression_rep;

pub fn get_type_rep(ty: &Option<Type>) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty),
        None => String::from("var"),
    }
}

pub fn get_typed_type_rep(ty: &Type) -> String {
    match &ty {
        Type::Local(_) => String::from("local"),
        Type::Plain(t) => String::from(t.name.value),
        Type::Array(t) => format!(
            "{}[{}]",
            get_typed_type_rep(&*t.base),
            get_expression_rep(&t.len)
        ),
        Type::Generic(_) => todo!(),
        Type::FunctionRef(p) => get_functionref_type_rep(p),
        Type::Struct(_) => todo!(),
        Type::Reference(t) => format!("{}&", get_typed_type_rep(&*t.base)),
        Type::Nullable(t) => format!("{} ornull", get_typed_type_rep(&*t.base)),
    }
}

fn get_functionref_type_rep(f: &sqparse::ast::FunctionRefType) -> String {
    let args_rep = get_functionref_args_rep(&f.params);
    let padding = if args_rep.len() > 0 { " " } else { "" }; // TODO: read from config
    format!(
        "{} functionref({padding}{args_rep}{padding})",
        get_boxed_type_rep(&f.return_type),
    )
}

fn get_boxed_type_rep(ty: &Option<Box<sqparse::ast::Type<'_>>>) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty),
        None => String::from("var"),
    }
}

fn get_functionref_args_rep(
    args: &sqparse::ast::SeparatedListTrailing0<sqparse::ast::FunctionRefParam>,
) -> String {
    format!(
        "{}",
        match args {
            Some(args) => format!(
                "{}{}{}",
                args.items
                    .iter()
                    .map(|(arg, _)| get_functionref_arg_rep(&arg))
                    .collect::<Vec<_>>()
                    .join(", "),
                if args.items.len() > 0 { ", " } else { "" },
                get_functionref_arg_rep(&args.last_item),
            ),
            None => String::from(""),
        }
    )
}

fn get_functionref_arg_rep(arg: &sqparse::ast::FunctionRefParam) -> String {
    format!(
        "{}{}",
        get_typed_type_rep(&arg.type_),
        match &arg.name {
            Some(name) => format!(" {}", name.value),
            None => String::from(""),
        }
    )
}

pub fn get_typedef_rep(def: &TypeDefinitionStatement) -> String {
    format!(
        "typedef {} {}",
        def.name.value,
        get_typed_type_rep(&def.type_)
    )
}
