use sqparse::ast::{Type, TypeDefinitionStatement};

use crate::{get_expression_rep, struct_rep::get_anon_struct_definition_rep, tokens::get_token};

pub fn get_type_rep(ty: &Option<Type>, depth: usize) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty, depth),
        None => String::from("var"),
    }
}

pub fn get_typed_type_rep(ty: &Type, depth: usize) -> String {
    match &ty {
        Type::Local(t) => get_token(t.local, "local"),
        Type::Var(t) => get_token(t.var, "var"),
        Type::Plain(t) => get_token(t.name.token, t.name.value),
        Type::Array(t) => format!(
            "{}{}{}{}",
            get_typed_type_rep(&*t.base, depth),
            get_token(t.open, "["),
            get_expression_rep(&t.len, depth),
            get_token(t.close, "]")
        ),
        Type::Generic(p) => get_generic_type_rep(p, depth),
        Type::FunctionRef(p) => get_functionref_type_rep(p, depth),
        Type::Struct(t) => get_anon_struct_definition_rep(t, depth),
        Type::Reference(t) => format!(
            "{}{}",
            get_typed_type_rep(&*t.base, depth),
            get_token(t.reference, "&")
        ),
        Type::Nullable(t) => format!(
            "{} {}",
            get_typed_type_rep(&*t.base, depth),
            get_token(t.ornull, "ornull")
        ),
    }
}

fn get_functionref_type_rep(f: &sqparse::ast::FunctionRefType, depth: usize) -> String {
    let args_rep = get_functionref_args_rep(&f.params, depth);
    let padding = if args_rep.len() > 0 { " " } else { "" }; // TODO: read from config
    format!(
        "{} {}{}{padding}{args_rep}{padding}{}",
        get_boxed_type_rep(&f.return_type, depth),
        get_token(f.functionref, "functionref"),
        get_token(f.open, "("),
        get_token(f.close, ")"),
    )
}

fn get_boxed_type_rep(ty: &Option<Box<sqparse::ast::Type<'_>>>, depth: usize) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty, depth),
        None => String::from("var"),
    }
}

fn get_functionref_args_rep(
    args: &sqparse::ast::SeparatedListTrailing0<sqparse::ast::FunctionRefParam>,
    depth: usize,
) -> String {
    format!(
        "{}",
        match args {
            Some(args) => format!(
                "{}{}",
                args.items
                    .iter()
                    .map(|(arg, comma)| format!(
                        "{}{} ",
                        get_functionref_arg_rep(&arg, depth),
                        get_token(comma, ",")
                    ))
                    .collect::<String>(),
                get_functionref_arg_rep(&args.last_item, depth),
            ),
            None => String::new(),
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
        "{} {} {}",
        get_token(def.typedef, "typedef"),
        def.name.value,
        get_typed_type_rep(&def.type_, depth)
    )
}

fn get_generic_type_rep(ty: &sqparse::ast::GenericType, depth: usize) -> String {
    format!(
        "{}{}{}{}",
        get_typed_type_rep(&*ty.base, depth),
        get_token(ty.open, "<"),
        get_generic_type_content_rep(&ty.params, depth),
        get_token(ty.close, ">"),
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

    // format!("{padding}{}{padding}", rep)
    format!(" {rep} ")
}
