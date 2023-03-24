use sqparse::ast::{Statement, SwitchCase, SwitchStatement};

use crate::{
    rep::{expressions::get_expression_rep, statements::get_full_statement_rep, tokens::get_token},
    utils::{get_lead, rep_starts_with_comment},
};

pub fn get_switch_rep(stm: &SwitchStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    let cases_rep = stm
        .cases
        .iter()
        .map(|case| get_case_rep(case, depth + 1))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{}{} {} {}\n{lead}{}\n{}\n{lead}{}",
        get_token(stm.switch, "switch", depth),
        get_token(stm.open_condition, "(", depth),
        get_expression_rep(&stm.condition, depth),
        get_token(stm.close_condition, ")", depth),
        get_token(stm.open_cases, "{", depth),
        cases_rep,
        get_token(stm.close_cases, "}", depth),
    )
}

fn get_case_rep(case: &SwitchCase, depth: usize) -> String {
    let case_lead = get_lead(depth);
    match &case.condition {
        sqparse::ast::SwitchCaseCondition::Default { default } => {
            let rep = format!(
                "{}{}{}",
                get_token(default, "default", depth),
                get_token(case.colon, ":", depth),
                if !case.body.is_empty() {
                    format!("\n{}", get_case_body_rep(&case.body, depth + 1))
                } else {
                    String::new()
                }
            );
            format!(
                "{}{rep}",
                if rep_starts_with_comment(&rep) {
                    ""
                } else {
                    &case_lead
                }
            )
        }
        sqparse::ast::SwitchCaseCondition::Case { case: c, value } => {
            let rep = format!(
                "{} {}{}{}",
                get_token(c, "case", depth),
                get_expression_rep(value, depth + 1),
                get_token(case.colon, ":", depth),
                if !case.body.is_empty() {
                    format!("\n{}", get_case_body_rep(&case.body, depth + 1))
                } else {
                    String::new()
                }
            );
            format!(
                "{}{rep}",
                if rep_starts_with_comment(&rep) {
                    ""
                } else {
                    &case_lead
                }
            )
        }
    }
}

fn get_case_body_rep(body: &[Statement], depth: usize) -> String {
    let lead = get_lead(depth);
    body.iter()
        .map(|body| {
            let rep = get_full_statement_rep(body, depth);
            format!(
                "{}{}",
                if rep_starts_with_comment(&rep) {
                    ""
                } else {
                    &lead
                },
                rep
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
