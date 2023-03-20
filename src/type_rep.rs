use sqparse::{
    ast::{Type, TypeDefinitionStatement},
    token::Token,
};

use crate::{
    get_config, get_expression_rep,
    struct_rep::get_anon_struct_definition_rep,
    tokens::{get_pre_token_lines, get_token},
    utils::get_optional_padding,
    var_rep::get_var_initializer_rep,
};

pub fn get_type_rep(ty: &Option<Type>, first_guaranteed_token: &Token, depth: usize) -> String {
    match ty {
        Some(ty) => get_typed_type_rep(ty, depth),
        None => format!("{}var", get_pre_token_lines(first_guaranteed_token, depth)),
    }
}

pub fn get_typed_type_rep(ty: &Type, depth: usize) -> String {
    match &ty {
        Type::Local(t) => get_token(t.local, "local", depth),
        Type::Var(t) => get_token(t.var, "var", depth),
        Type::Plain(t) => get_token(t.name.token, t.name.value, depth),
        Type::Array(t) => format!(
            "{}{}{}{}",
            get_typed_type_rep(&*t.base, depth),
            get_token(t.open, "[", depth),
            get_expression_rep(&t.len, depth),
            get_token(t.close, "]", depth)
        ),
        Type::Generic(p) => get_generic_type_rep(p, depth),
        Type::FunctionRef(p) => get_functionref_type_rep(p, depth),
        Type::Struct(t) => get_anon_struct_definition_rep(t, depth),
        Type::Reference(t) => format!(
            "{}{}",
            get_typed_type_rep(&*t.base, depth),
            get_token(t.reference, "&", depth)
        ),
        Type::Nullable(t) => format!(
            "{} {}",
            get_typed_type_rep(&*t.base, depth),
            get_token(t.ornull, "ornull", depth)
        ),
    }
}

fn get_functionref_type_rep(f: &sqparse::ast::FunctionRefType, depth: usize) -> String {
    let args_rep = get_functionref_args_rep(&f.params, depth);
    let padding = get_optional_padding(
        args_rep.len() > get_config().lock().unwrap().functionref_oneliner_args_max,
    );
    format!(
        "{} {}{}{padding}{args_rep}{padding}{}",
        get_boxed_type_rep(&f.return_type, depth),
        get_token(f.functionref, "functionref", depth),
        get_token(f.open, "(", depth),
        get_token(f.close, ")", depth),
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
                        get_token(comma, ",", depth)
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
        "{}{}{}",
        get_typed_type_rep(&arg.type_, depth),
        match &arg.name {
            Some(name) => format!(" {}", name.value),
            None => String::new(),
        },
        match &arg.initializer {
            Some(initializer) => get_var_initializer_rep(initializer, depth),
            None => String::new(),
        }
    )
}

pub fn get_typedef_rep(def: &TypeDefinitionStatement, depth: usize) -> String {
    format!(
        "{} {} {}",
        get_token(def.typedef, "typedef", depth),
        def.name.value,
        get_typed_type_rep(&def.type_, depth)
    )
}

fn get_generic_type_rep(ty: &sqparse::ast::GenericType, depth: usize) -> String {
    format!(
        "{}{}{}{}",
        get_typed_type_rep(&*ty.base, depth),
        get_token(ty.open, "<", depth),
        get_generic_type_content_rep(&ty.params, depth),
        get_token(ty.close, ">", depth),
    )
}

fn get_generic_type_content_rep(
    types: &sqparse::ast::SeparatedListTrailing1<Type>,
    depth: usize,
) -> String {
    let mut padding = get_optional_padding(get_config().lock().unwrap().non_generic_type_padding);
    let rep = format!(
        "{}{}",
        types
            .items
            .iter()
            .map(|(t, comma)| format!(
                "{}{} ",
                get_typed_type_rep(t, depth),
                get_token(comma, ",", depth)
            ))
            .collect::<String>(),
        get_typed_type_rep(&*types.last_item, depth)
    );

    for item in &types.items {
        if let (Type::Generic(_), _) = item {
            padding = " ";
        }
    }

    if matches!(&*types.last_item, Type::Generic(_)) {
        padding = " "; // This is required to compile since right bit shift (>>) will be lexed before any types
    }

    format!("{padding}{}{padding}", rep)
}
