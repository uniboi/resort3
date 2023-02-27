use sqparse::ast::{Type, TypeDefinitionStatement};

use crate::get_expression_rep;

pub fn get_type_rep(ty: &Option<Type>, depth: usize) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty, depth),
        None => String::from("var"),
    }
}

pub fn get_typed_type_rep(ty: &Type, depth: usize) -> String {
    match &ty {
        Type::Local(_) => String::from("local"),
        Type::Plain(t) => String::from(t.name.value),
        Type::Array(t) => format!(
            "{}[{}]",
            get_typed_type_rep(&*t.base, depth),
            get_expression_rep(&t.len, depth)
        ),
        Type::Generic(p) => get_generic_type_rep(p, depth),
        Type::FunctionRef(p) => get_functionref_type_rep(p, depth),
        Type::Struct(_) => todo!(),
        Type::Reference(t) => format!("{}&", get_typed_type_rep(&*t.base, depth)),
        Type::Nullable(t) => format!("{} ornull", get_typed_type_rep(&*t.base, depth)),
    }
}

fn get_functionref_type_rep(f: &sqparse::ast::FunctionRefType, depth: usize) -> String {
    let args_rep = get_functionref_args_rep(&f.params, depth);
    let padding = if args_rep.len() > 0 { " " } else { "" }; // TODO: read from config
    format!(
        "{} functionref({padding}{args_rep}{padding})",
        get_boxed_type_rep(&f.return_type, depth),
    )
}

fn get_boxed_type_rep(ty: &Option<Box<sqparse::ast::Type<'_>>>, depth: usize) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty,depth),
        None => String::from("var"),
    }
}

fn get_functionref_args_rep(
    args: &sqparse::ast::SeparatedListTrailing0<sqparse::ast::FunctionRefParam>, depth: usize
) -> String {
    format!(
        "{}",
        match args {
            Some(args) => format!(
                "{}{}{}",
                args.items
                    .iter()
                    .map(|(arg, _)| get_functionref_arg_rep(&arg, depth))
                    .collect::<Vec<_>>()
                    .join(", "),
                if args.items.len() > 0 { ", " } else { "" },
                get_functionref_arg_rep(&args.last_item, depth),
            ),
            None => String::from(""),
        }
    )
}

fn get_functionref_arg_rep(arg: &sqparse::ast::FunctionRefParam, depth: usize) -> String {
    format!(
        "{}{}",
        get_typed_type_rep(&arg.type_, depth),
        match &arg.name {
            Some(name) => format!(" {}", name.value),
            None => String::from(""),
        }
    )
}

pub fn get_typedef_rep(def: &TypeDefinitionStatement, depth: usize) -> String {
    format!(
        "typedef {} {}",
        def.name.value,
        get_typed_type_rep(&def.type_, depth)
    )
}

fn get_generic_type_rep(ty: &sqparse::ast::GenericType, depth: usize) -> String {
    format!(
        "{}<{}>",
        get_typed_type_rep(&*ty.base, depth),
        get_generic_type_content_rep(&ty.params, depth)
    )
}

fn get_generic_type_content_rep(
    types: &sqparse::ast::SeparatedListTrailing1<Type>,
    depth: usize,
) -> String {
    let mut padding = ""; // TODO: Read from config
    let rep = format!(
        "{}{}{}",
        types
            .items
            .iter()
            .map(|(t, _)| get_typed_type_rep(t, depth))
            .collect::<Vec<_>>()
            .join(", "),
        if types.items.len() > 0 { ", " } else { "" },
        get_typed_type_rep(&*types.last_item, depth)
    );

    if matches!(&*types.last_item, Type::Generic(_)) {
        padding = " "; // This is required to compile since right bit shift (>>) will be lexed before any types
    }

    format!("{padding}{}{padding}", rep)
}
