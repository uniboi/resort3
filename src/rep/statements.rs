use sqparse::ast::{Statement, StatementType};

use crate::{
    get_config,
    rep::{
		preprocessed::get_preprocessed_if_rep,
        block_rep::get_block_rep,
        class_rep::get_class_statement_rep,
        constructor_res::get_constructor_def_rep,
        enum_rep::get_enum_rep,
        expressions::get_expression_rep,
        for_rep::get_for_rep,
        foreach_rep::get_foreach_rep,
        function_rep::get_function_definition_rep,
        global_rep::get_global_rep,
        if_rep::get_if_rep,
        struct_rep::get_struct_definition_rep,
        switch_rep::get_switch_rep,
        tokens::get_token,
        try_rep::{get_try_rep, throw_rep},
        type_rep::get_typedef_rep,
        var_rep::{get_const_rep, get_var_definition_list_rep},
        while_rep::{get_do_while_rep, get_while_rep},
        yields_rep::{get_delaythread_rep, get_return_rep, get_yield_rep},
    },
    utils::{clear_whitespace_lines, get_lead, rep_starts_with_comment},
};

pub fn get_full_statement_rep(statement: &Statement, depth: usize) -> String {
    get_statement_rep(&statement.ty, depth)
}

pub fn get_statement_rep(statement: &StatementType, depth: usize) -> String {
    let mut add_semicolon = match &statement {
        StatementType::Empty(_) => false,
        StatementType::Block(_) => false,
        StatementType::While(_) => false,
        StatementType::Switch(_) => false,
        StatementType::For(_) => false,
        StatementType::ConstructorDefinition(_) => false,
        StatementType::FunctionDefinition(_) => false,
        StatementType::ClassDefinition(_) => false,
        StatementType::TryCatch(_) => false,
        StatementType::EnumDefinition(_) => false,
        StatementType::StructDefinition(_) => false,
        StatementType::Preprocessed(_) => false,
        StatementType::PreprocessedDocumentation(_) => false,
        _ => true,
    };

    let rep: String = match &statement {
        StatementType::Empty(_) => todo!(),
        StatementType::Block(p) => get_block_rep(p, depth),
        StatementType::If(p) => get_if_rep(p, depth),
        StatementType::While(p) => get_while_rep(p, depth),
        StatementType::DoWhile(p) => get_do_while_rep(p, depth),
        StatementType::Switch(p) => get_switch_rep(p, depth),
        StatementType::For(p) => get_for_rep(p, depth),
        StatementType::Foreach(p) => get_foreach_rep(p, depth),
        StatementType::Break(p) => get_token(p.break_, "break", depth),
        StatementType::Continue(p) => get_token(p.continue_, "continue", depth),
        StatementType::Return(p) => get_return_rep(p, depth),
        StatementType::Yield(p) => get_yield_rep(p, depth),
        StatementType::VarDefinition(p) => get_var_definition_list_rep(p, depth),
        StatementType::ConstructorDefinition(p) => get_constructor_def_rep(p, depth),
        StatementType::FunctionDefinition(p) => get_function_definition_rep(p, depth),
        StatementType::ClassDefinition(p) => get_class_statement_rep(p, depth),
        StatementType::TryCatch(p) => get_try_rep(p, depth),
        StatementType::Throw(p) => throw_rep(p, depth),
        StatementType::Const(p) => get_const_rep(p, depth),
        StatementType::EnumDefinition(p) => get_enum_rep(p, depth),
        StatementType::Expression(p) => get_expression_rep(&*p.value, depth),
        StatementType::Thread(p) => format!(
            "{} {}",
            get_token(p.thread, "thread", depth),
            get_expression_rep(&*p.value, depth)
        ),
        StatementType::DelayThread(p) => get_delaythread_rep(p, depth),
        StatementType::WaitThread(p) => get_token(p.wait_thread, "waitthread", depth),
        StatementType::WaitThreadSolo(p) => get_token(p.wait_thread_solo, "waitthreadsolo", depth),
        StatementType::Wait(p) => format!(
            "{} {}",
            get_token(p.wait, "wait", depth),
            get_expression_rep(&*p.value, depth)
        ),
        StatementType::StructDefinition(p) => get_struct_definition_rep(p, depth),
        StatementType::TypeDefinition(p) => get_typedef_rep(p, depth),
        StatementType::Global(p) => get_global_rep(p, depth),
        StatementType::GlobalizeAllFunctions(p) => {
            get_token(p.globalize_all_functions, "globalize_all_functions", depth)
        }
        StatementType::Untyped(p) => get_token(p.untyped, "untyped", depth),
        StatementType::Preprocessed(p) => get_preprocessed_if_rep(
            p.as_ref(),
            &|contents, depth| {
                contents
                    .iter()
                    .map(|c| {
                        let rep = get_full_statement_rep(&c, depth + 1);
                        let raw = format!(
                            "{}{}",
                            if rep_starts_with_comment(&rep) {
                                String::new()
                            } else {
                                get_lead(depth + 1)
                            },
                            rep
                        );
                        clear_whitespace_lines(raw.split("\n"), depth + 1)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            depth,
        ),
        StatementType::PreprocessedDocumentation(p) => format!(
            "{}{} \"{}\"{} \"{}\" {}",
            get_token(p.document, "#document", depth),
            get_token(p.open, "(", depth),
            get_token(p.property_token, p.property, depth),
            get_token(p.seperator, ",", depth),
            get_token(p.help_text_token, p.help_text, depth),
            get_token(p.close, ")", depth)
        ),
    };
    format!(
        "{rep}{}",
        if get_config().lock().unwrap().semicolons && add_semicolon {
            ";"
        } else {
            ""
        }
    )
}
