use sqparse::ast::{Statement, SwitchCase, SwitchStatement};

use crate::{get_expression_rep, get_full_statement_rep, tokens::get_token, utils::get_lead};

pub fn get_switch_rep(stm: &SwitchStatement, depth: usize) -> String {
    let lead = get_lead(depth);
    format!(
        "{}{} {} {}\n{lead}{}\n{}\n{lead}{}",
        get_token(stm.switch, "switch", depth),
        get_token(stm.open_condition, "(", depth),
        get_expression_rep(&*stm.condition, depth),
        get_token(stm.close_condition, ")", depth),
        get_token(stm.open_cases, "{", depth),
        stm.cases
            .iter()
            .map(|case| get_case_rep(case, depth + 1))
            .collect::<Vec<_>>()
            .join("\n"),
        get_token(stm.close_cases, "}", depth),
    )
}

fn get_case_rep(case: &SwitchCase, depth: usize) -> String {
    let case_lead = get_lead(depth);
    let body_lead = get_lead(depth + 1);
    match &case.condition {
        sqparse::ast::SwitchCaseCondition::Default { default } => format!(
            "{case_lead}{}{}{}",
            get_token(default, "default", depth),
            get_token(case.colon, ":", depth),
            if case.body.len() > 0 {
                format!("\n{}", get_case_body_rep(&case.body, depth + 1))
            } else {
                String::new()
            }
        ),
        sqparse::ast::SwitchCaseCondition::Case { case: c, value } => {
            format!(
                "{case_lead}{} {}{}{}",
                get_token(c, "case", depth),
                get_expression_rep(&*value, depth + 1),
                get_token(case.colon, ":", depth),
                if case.body.len() > 0 {
                    format!("\n{}", get_case_body_rep(&case.body, depth + 1))
                } else {
                    String::new()
                }
            )
        }
    }
}

fn get_case_body_rep(body: &Vec<Statement>, depth: usize) -> String {
    let lead = get_lead(depth);
	body.iter()
        .map(|body| format!("{}{}", lead, get_full_statement_rep(&body, depth)))
        .collect::<Vec<_>>()
        .join("\n")
}
