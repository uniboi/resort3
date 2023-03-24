use sqparse::{
    ast::{
        CallArgument, CallExpression, FunctionDefinition, FunctionDefinitionStatement,
        FunctionEnvironment, FunctionExpression, FunctionParam, FunctionParams, Identifier,
        SeparatedList1, SeparatedListTrailing1, StatementType, Type,
    },
    token::Token,
};

use crate::{
    rep::{
        block_rep::get_empty_block,
        expressions::get_expression_rep,
        statements::get_statement_rep,
        tokens::{get_headless_token, get_token},
        type_rep::{get_type_rep, get_typed_type_rep},
    },
    utils::{get_lead, rep_includes_single_line_comment},
};

pub fn get_function_rep(f: &FunctionExpression, depth: usize) -> String {
    let return_type = match &f.return_type {
        Some(p) => format!("{} ", get_typed_type_rep(p, depth)),
        None => String::new(),
    };
    format!(
        "{return_type}{}{}{}{}{}{}{}",
        get_token(f.function, "function", depth),
        get_environment_rep(&f.definition.environment, depth),
        get_token(f.definition.open, "(", depth),
        get_function_param_rep(&f.definition.params, depth),
        get_token(f.definition.close, ")", depth),
        match &f.definition.captures {
            Some(p) => get_capture_rep(p, depth),
            None => String::new(),
        },
        get_statement_rep(&f.definition.body, depth)
    )
}

fn get_capture_rep(capture: &sqparse::ast::FunctionCaptures, depth: usize) -> String {
    format!(
        " {} {}{}{}",
        get_token(capture.colon, ":", depth),
        get_token(capture.open, "(", depth),
        match &capture.names {
            Some(idens) => format!(
                " {}{}{} ",
                idens
                    .items
                    .iter()
                    .map(|(identifier, _)| identifier.value)
                    .collect::<Vec<_>>()
                    .join(", "),
                if idens.items.is_empty() { " " } else { ", " },
                idens.last_item.value
            ),
            None => String::new(),
        },
        get_token(capture.close, ")", depth),
    )
}

fn get_function_param_rep(args: &FunctionParams, depth: usize) -> String {
    let lead = get_lead(depth);
    let inline_rep = match args {
        FunctionParams::NonVariable { params } => match params {
            Some(params) => format!(
                " {} ",
                get_all_typed_args_rep(&params.items, &params.last_item, depth)
            ),
            None => String::new(),
        },
        FunctionParams::EmptyVariable { vararg } => get_token(vararg, "...", depth),
        FunctionParams::NonEmptyVariable {
            comma,
            vararg,
            params,
        } => format!(
            " {}{} {} ",
            get_all_typed_args_rep(&params.items, &params.last_item, depth),
            get_token(comma, ",", depth),
            get_token(vararg, "...", depth),
        ),
    };

    if inline_rep.find('\n').is_some() || rep_includes_single_line_comment(&inline_rep) {
        return format!("{}\n{lead}", get_multiline_function_params_rep(args, depth));
    }

    inline_rep
}

fn get_multiline_function_params_rep(args: &FunctionParams, depth: usize) -> String {
    let param_lead = get_lead(depth + 1);
    match args {
        FunctionParams::NonVariable { params } => match params {
            Some(params) => get_multiline_function_params_value_rep(params, depth),
            None => String::new(),
        },
        FunctionParams::EmptyVariable { vararg: _ } => {
            todo!("only vargs available in multiline function definition")
        } // There should be no way this can happen
        FunctionParams::NonEmptyVariable {
            params,
            comma,
            vararg,
        } => format!(
            "{}{}\n{param_lead}{}",
            get_multiline_function_params_value_non_trailing_rep(params, depth),
            get_token(comma, ",", depth),
            get_token(vararg, "...", depth)
        ),
    }
}

fn get_multiline_function_params_value_rep(
    params: &SeparatedListTrailing1<FunctionParam>,
    depth: usize,
) -> String {
    let trailing_comma = false; // TODO: read from config

    let trailing_comma_rep = if trailing_comma { "," } else { "" };

    format!(
        "{}{}",
        get_multiline_function_params_value_internal(
            &params
                .items
                .iter()
                .map(|(param, seperator)| (param, seperator))
                .collect::<Vec<_>>(),
            &params.last_item,
            depth
        ),
        match params.trailing {
            Some(comma) => get_token(comma, trailing_comma_rep, depth),
            None => String::from(trailing_comma_rep),
        },
    )
}

fn get_multiline_function_params_value_non_trailing_rep(
    params: &SeparatedList1<FunctionParam>,
    depth: usize,
) -> String {
    get_multiline_function_params_value_internal(
        &params
            .items
            .iter()
            .map(|(param, seperator)| (param, seperator))
            .collect::<Vec<_>>(),
        &params.last_item,
        depth,
    )
}

fn get_multiline_function_params_value_internal(
    params: &[(&FunctionParam, &&Token)],
    last_param: &FunctionParam,
    depth: usize,
) -> String {
    let param_lead = get_lead(depth + 1);
    format!(
        "{}\n{param_lead}{}",
        params
            .iter()
            .map(|(param, seperator)| {
                format!(
                    "\n{param_lead}{}{}",
                    get_typed_arg_rep(param, depth),
                    get_token(seperator, ",", depth)
                )
            })
            .collect::<String>(),
        get_typed_arg_rep(last_param, depth)
    )
}

fn get_typed_arg_rep(arg: &FunctionParam, depth: usize) -> String {
    format!(
        "{} {}{}",
        get_type_rep(&arg.type_, arg.name.token, depth),
        get_headless_token(arg.name.token, arg.name.value, depth), // TODO: idk what I did but this seems wrong
        match &arg.initializer {
            Some(init) => format!(" = {}", get_expression_rep(&init.value, depth + 1)),
            None => String::from(""),
        }
    )
}

fn get_all_typed_args_rep(
    args: &[(FunctionParam<'_>, &sqparse::token::Token<'_>)],
    last_arg: &FunctionParam,
    depth: usize,
) -> String {
    format!(
        "{}{}",
        args.iter()
            .map(|(arg, comma)| format!(
                "{}{} ",
                get_typed_arg_rep(arg, depth),
                get_token(comma, ",", depth)
            ))
            .collect::<String>(),
        get_typed_arg_rep(last_arg, depth)
    )
}

pub fn get_function_definition_rep(f: &FunctionDefinitionStatement, depth: usize) -> String {
    format!(
        "{} {} {}{}{}",
        get_type_rep(&f.return_type, f.function, depth),
        get_headless_token(f.function, "function", depth),
        f.name
            .items
            .iter()
            .map(|(name, namespace)| format!("{}{}", name.value, get_token(namespace, "::", depth)))
            .collect::<String>(),
        f.name.last_item.value,
        get_function_def_rep(&f.definition, depth)
    )
}

pub fn get_function_def_rep(def: &FunctionDefinition, depth: usize) -> String {
    let (empty_body, empty_block) = match &*def.body {
        StatementType::Block(b) => (b.statements.is_empty(), get_empty_block(b, depth)),
        _ => (
            false,
            String::from("/* formatting error: expected empty block */"), // bruh moment
        ),
    };

    format!(
        "{}{}{}{}{}{}{}",
        get_environment_rep(&def.environment, depth),
        get_token(def.open, "(", depth),
        get_function_param_rep(&def.params, depth),
        get_token(def.close, ")", depth),
        match &def.captures {
            Some(capture) => get_capture_rep(capture, depth),
            None => "".to_owned(),
        },
        get_lead(depth),
        if empty_body {
            format!(" {empty_block}")
        } else {
            format!("\n{}", get_statement_rep(&def.body, depth))
        }
    )
}

fn get_environment_rep(env: &Option<FunctionEnvironment>, depth: usize) -> String {
    match &env {
        Some(env) => format!(
            "{} {} {}",
            get_token(env.open, "[", depth),
            get_expression_rep(&env.value, depth),
            get_token(env.close, "]", depth),
        ),
        None => String::new(),
    }
}

pub fn get_call_rep(p: &CallExpression, depth: usize) -> String {
    format!(
        "{}{}{}{}",
        get_expression_rep(&p.function, depth),
        get_token(p.open, "(", depth),
        get_call_params_rep(&p.arguments, depth),
        get_token(p.close, ")", depth),
    )
}

fn get_call_params_rep(args: &Vec<CallArgument>, depth: usize) -> String {
    let max_oneliner_args = 5; // TODO: read from config

    if args.is_empty() {
        return String::new();
    }
    let rep = format!(
        " {} ",
        args.iter()
            .map(|arg| {
                format!(
                    "{}{}",
                    get_expression_rep(&arg.value, depth),
                    match &arg.comma {
                        Some(token) => get_token(token, ", ", depth),
                        None => String::new(),
                    }
                )
            })
            .collect::<String>()
    );

    // call expressions with newlines should be multiline
    if args.len() >= max_oneliner_args || rep.find('\n').is_some() {
        let lead = get_lead(depth + 1);
        return format!(
            "\n{}\n{}",
            args.iter()
                .map(|arg| format!(
                    "{lead}{}{}",
                    get_expression_rep(&arg.value, depth + 1),
                    match &arg.comma {
                        Some(token) => get_token(token, ",", depth),
                        None => String::new(),
                    }
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            get_lead(depth)
        );
    }
    return rep;
}

pub fn get_fragmented_named_function_rep(
    return_type: &Option<Type>,
    function: &Token,
    name: &Identifier,
    definition: &FunctionDefinition,
    depth: usize,
) -> String {
    format!(
        "{}{} {}{}",
        match &return_type {
            Some(ty) => get_typed_type_rep(ty, depth),
            None => String::new(),
        },
        get_token(function, "function", depth),
        name.value,
        get_function_def_rep(definition, depth)
    )
}
