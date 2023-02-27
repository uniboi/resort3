use crate::get_expression_rep;

pub fn get_literal_rep(exp: &sqparse::ast::LiteralExpression) -> String {
    match &exp.literal {
        sqparse::token::LiteralToken::Int(v, base) => get_integer_rep(exp, v, base),
        sqparse::token::LiteralToken::Char(c) => format!("'{c}'"),
        sqparse::token::LiteralToken::Float(f) => {
            let start_at_dot = false; // TODO: read from config
            let rep = format!("{f}");

            if start_at_dot {
                rep[if &exp.token.range.end - &exp.token.range.start < rep.len() {
                    1
                } else {
                    0
                } as usize..]
                    .to_owned()
            } else {
                rep
            }
        }
        sqparse::token::LiteralToken::String(s) => get_string_rep(s),
    }
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
    let padding = " "; // TODO: read from config
    format!(
        "<{padding}{}, {}, {}{padding}>",
        get_expression_rep(&exp.x, depth),
        get_expression_rep(&exp.y, depth),
        get_expression_rep(&exp.z, depth)
    )
}
