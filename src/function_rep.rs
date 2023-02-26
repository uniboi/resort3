use sqparse::ast::{
    CallExpression, Expression, FunctionDefinitionStatement, FunctionExpression, FunctionParam,
    FunctionParams, SeparatedListTrailing1,
};

use crate::{
    get_expression_rep, get_statement_rep,
    type_rep::{get_type_rep, get_typed_type_rep},
};

pub fn get_function_rep(f: &FunctionExpression) -> String {
    let return_type = match &f.return_type {
        Some(p) => format!("{} ", get_typed_type_rep(p)),
        None => String::new(),
    };
    format!(
        "{return_type}function({}){}{}",
        get_function_param_rep(&f.definition.params),
        match &f.definition.captures {
            Some(p) => get_capture_rep(p),
            None => "".to_owned(),
        },
        get_statement_rep(&*f.definition.body)
    )
}

fn get_capture_rep(capture: &sqparse::ast::FunctionCaptures) -> String {
    format!(
        " : ({})",
        match &capture.names {
            Some(idens) => format!(
                " {}{}{} ",
                idens
                    .items
                    .iter()
                    .map(|(identifier, _)| identifier.value)
                    .collect::<Vec<_>>()
                    .join(", "),
                if idens.items.len() > 0 { ", " } else { " " },
                idens.last_item.value
            ),
            None => String::new(),
        }
    )
}

fn get_function_param_rep(args: &FunctionParams) -> String {
    match args {
        FunctionParams::NonVariable { params } => match params {
            Some(params) => get_all_typed_args_rep(&params.items, &params.last_item),
            None => "".to_owned(),
        },
        FunctionParams::EmptyVariable { vararg: _ } => " ... ".to_owned(),
        FunctionParams::NonEmptyVariable {
            comma: _,
            vararg: _,
            params,
        } => format!(
            "{}, ... ",
            get_all_typed_args_rep(&params.items, &params.last_item)
        ),
    }
}

fn get_typed_arg_rep(arg: &FunctionParam) -> String {
    format!(
        "{} {}{}",
        get_type_rep(&arg.type_),
        arg.name.value,
        match &arg.initializer {
            Some(init) => format!(" = {}", get_expression_rep(&*init.value)),
            None => String::from(""),
        }
    )
}

fn get_all_typed_args_rep(
    args: &Vec<(FunctionParam<'_>, &sqparse::token::Token<'_>)>,
    last_arg: &FunctionParam,
) -> String {
    format!(
        " {}{}{} ",
        args.iter()
            .map(|(arg, _)| get_typed_arg_rep(arg))
            .collect::<Vec<_>>()
            .join(", "),
        if args.len() > 0 { ", " } else { "" },
        get_typed_arg_rep(last_arg)
    )
}

pub fn get_function_definition_rep(f: &FunctionDefinitionStatement) -> String {
    format!(
        "{} function {}({}){}{}",
        get_type_rep(&f.return_type),
        f.name.last_item.value,
        get_function_param_rep(&f.definition.params),
        match &f.definition.captures {
            Some(capture) => get_capture_rep(capture),
            None => "".to_owned(),
        },
        get_statement_rep(&f.definition.body)
    )
}

pub fn get_call_rep(p: &CallExpression) -> String {
    format!("{}({})", get_expression_rep(&*p.function), get_call_params_rep(&p.arguments))
}

fn get_call_params_rep(args: &Option<SeparatedListTrailing1<Expression>>) -> String {
    match args {
        Some(list) => format!(
            " {}{}{} ",
            list.items
                .iter()
                .map(|(expression, _)| get_expression_rep(expression))
                .collect::<Vec<_>>()
                .join(", "),
            if list.items.len() > 0 { ", " } else { "" },
            get_expression_rep(&list.last_item)
        ),
        None => String::from(""),
    }
}
