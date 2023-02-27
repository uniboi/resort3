use sqparse::ast::GlobalStatement;

use crate::var_rep::{get_var_initializer_rep, get_const_rep};

pub fn get_global_rep(statement: &GlobalStatement, depth: usize) -> String {
    let global_rep = match &statement.definition {
        sqparse::ast::GlobalDefinition::Function { function: _, name } => {
            format!("function {}", name.value)
        }
        sqparse::ast::GlobalDefinition::UntypedVar { name, initializer } => {
            const EXPLICIT_TYPES: bool = false; // TODO: read from config
            let ty = if EXPLICIT_TYPES { "TODO_EXPLICIT_TYPES " } else { "" };
            format!("{ty}{}{}", name.value, get_var_initializer_rep(initializer, depth))
        }
        sqparse::ast::GlobalDefinition::TypedVar(_) => todo!(),
        sqparse::ast::GlobalDefinition::Const(p) => get_const_rep(p, depth),
        sqparse::ast::GlobalDefinition::Enum(_) => todo!(),
        sqparse::ast::GlobalDefinition::Class(_) => todo!(),
        sqparse::ast::GlobalDefinition::Struct(_) => todo!(),
        sqparse::ast::GlobalDefinition::Type(_) => todo!(),
    };
    format!("global {global_rep}")
}