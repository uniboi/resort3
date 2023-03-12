use sqparse::ast::{SwitchCase, SwitchStatement};

use crate::{get_expression_rep, get_statement_rep, tokens::get_token, utils::get_lead};

pub fn get_switch_rep(stm: &SwitchStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "switch{} {} {}\n{lead}{{\n{}\n{lead}}}",
        get_token(stm.open_condition, "("),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close_condition, ")"),
        stm.cases
            .iter()
            .map(|case| get_case_rep(case, depth + 1))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn get_case_rep(case: &SwitchCase, depth: usize) -> String {
    let case_lead = get_lead(depth);
    let body_lead = get_lead(depth + 1);
    match &case.condition {
        sqparse::ast::SwitchCaseCondition::Default { default: _ } => format!(
            "{case_lead}default:\n{body_lead}{}",
            case.body
                .iter()
                .map(|body| get_statement_rep(&body.ty, depth + 1))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        sqparse::ast::SwitchCaseCondition::Case { case: _, value } => {
            format!(
                "{case_lead}case {}:\n{body_lead}{}",
                get_expression_rep(&*value, depth + 1),
                case.body
                    .iter()
                    .map(|body| get_statement_rep(&body.ty, depth + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}
