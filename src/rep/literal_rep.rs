use crate::{get_config, rep::expressions::get_expression_rep, utils::get_optional_padding};

use super::tokens::get_token;

pub fn get_literal_rep(exp: &sqparse::ast::LiteralExpression, depth: usize) -> String {
    get_token(
        exp.token,
        &match &exp.literal {
            sqparse::token::LiteralToken::Int(v, base) => get_integer_rep(exp, v, base),
            sqparse::token::LiteralToken::Char(c) => format!("'{c}'"),
            sqparse::token::LiteralToken::Float(f) => {
                let rep = format!("{f}{}", if f.fract() == 0.0 { ".0" } else { "" });
                if get_config().lock().unwrap().trim_float && f < &1.0 {
                    rep[1..].to_owned()
                } else {
                    rep
                }
                .to_string()
            }
            sqparse::token::LiteralToken::String(s) => get_string_rep(s),
        },
        depth,
    )
}

fn get_integer_rep(
    exp: &sqparse::ast::LiteralExpression,
    value: &i64,
    base: &sqparse::token::LiteralBase,
) -> String {
    match base {
        sqparse::token::LiteralBase::Decimal => format!("{value}"),
        sqparse::token::LiteralBase::Octal => {
            let oct = format!("{value:o}");
            let preamble = "0".repeat(&exp.token.range.end - &exp.token.range.start - oct.len());
            format!("{preamble}{oct}")
        }
        sqparse::token::LiteralBase::Hexadecimal => {
            let prefix = "0x";
            let hex = format!("{value:X}");
            let preamble = "0"
                .repeat(&exp.token.range.end - &exp.token.range.start - hex.len() - prefix.len());
            format!("{prefix}{preamble}{hex}")
        }
    }
}

fn get_string_rep(exp: &sqparse::token::StringToken) -> String {
    match exp {
        sqparse::token::StringToken::Literal(s) => format!("\"{s}\""),
        sqparse::token::StringToken::Verbatim(v) => format!("@\"{v}\""),
        sqparse::token::StringToken::Asset(a) => format!("$\"{a}\""),
    }
}

pub fn get_vector_rep(exp: &sqparse::ast::VectorExpression, depth: usize) -> String {
    let padding = get_optional_padding(get_config().lock().unwrap().vector_padding);
    format!(
        "{}{padding}{}{} {}{} {}{padding}{}",
        get_token(exp.open, "<", depth),
        get_expression_rep(&exp.x, depth),
        get_token(exp.comma_1, ",", depth),
        get_expression_rep(&exp.y, depth),
        get_token(exp.comma_2, ",", depth),
        get_expression_rep(&exp.z, depth),
        get_token(exp.close, ">", depth),
    )
}
