mod binary_rep;
mod function_rep;
mod global_rep;
mod literal_rep;
mod property_rep;
mod try_rep;
mod type_rep;
mod var_rep;
mod yields_rep;
mod block_rep;
mod utils;

use binary_rep::get_binary_rep;
use block_rep::get_block_rep;
use function_rep::{get_function_definition_rep, get_function_rep, get_call_rep};
use global_rep::get_global_rep;
use literal_rep::{get_literal_rep, get_vector_rep};
use property_rep::get_property_rep;
use sqparse::{
    ast::{Expression, StatementType},
    parse, tokenize, Flavor,
};
use try_rep::throw_rep;
use type_rep::{get_typed_type_rep, get_typedef_rep};
use var_rep::{get_const_rep, get_var_definition_list_rep};
use yields_rep::{get_delaythread_rep, get_return_rep, get_yield_rep};

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .expect("no path to directory or file was provided");
    println!("// Automatically generated from \"{}\"", path);

    let source = fs::read_to_string(path).expect("Failed reading file");

    let tokens = tokenize(&source, Flavor::SquirrelRespawn).unwrap();
    let ast = parse(&tokens).unwrap();

    // println!("{ast:#?}")
    for statement in ast.statements {
        println!("{}", get_statement_rep(&statement.ty, 0))
    }
}

fn get_statement_rep(statement: &StatementType, depth: usize) -> String {
    let rep: String = match &statement {
        StatementType::Empty(_) => todo!(),
        StatementType::Block(p) => get_block_rep(p, depth),
        StatementType::If(_) => todo!(),
        StatementType::While(_) => todo!(),
        StatementType::DoWhile(_) => todo!(),
        StatementType::Switch(_) => todo!(),
        StatementType::For(_) => todo!(),
        StatementType::Foreach(_) => todo!(),
        StatementType::Break(_) => String::from("break"),
        StatementType::Continue(_) => String::from("continue"),
        StatementType::Return(p) => get_return_rep(p, depth),
        StatementType::Yield(p) => get_yield_rep(p, depth),
        StatementType::VarDefinition(p) => get_var_definition_list_rep(p, depth),
        StatementType::ConstructorDefinition(_) => todo!(),
        StatementType::FunctionDefinition(p) => get_function_definition_rep(p, depth),
        StatementType::ClassDefinition(_) => todo!(),
        StatementType::TryCatch(_) => todo!(),
        StatementType::Throw(p) => throw_rep(p, depth),
        StatementType::Const(p) => get_const_rep(p, depth),
        StatementType::EnumDefinition(_) => todo!(),
        StatementType::Expression(p) => get_expression_rep(&*p.value, depth),
        StatementType::Thread(_) => todo!(),
        StatementType::DelayThread(p) => get_delaythread_rep(p, depth),
        StatementType::WaitThread(_) => String::from("waitthread"),
        StatementType::WaitThreadSolo(_) => String::from("waitthreadsolo"),
        StatementType::Wait(p) => format!("wait {}", get_expression_rep(&*p.value, depth)),
        StatementType::StructDefinition(_) => todo!(),
        StatementType::TypeDefinition(p) => get_typedef_rep(p, depth),
        StatementType::Global(p) => get_global_rep(p, depth),
        StatementType::GlobalizeAllFunctions(_) => String::from("globalize_all_functions"),
        StatementType::Untyped(_) => String::from("untyped"),
    };
    format!("{rep}")
}

// EXPRESSIONS

fn get_expression_rep(expression: &Expression, depth: usize) -> String {
    match expression {
        Expression::Parens(p) => format!("( {} )", get_expression_rep(&*p.value, depth)),
        Expression::Literal(p) => get_literal_rep(p),
        Expression::Var(p) => String::from(p.name.value),
        Expression::RootVar(p) => format!("::{}", p.name.value),
        Expression::Index(p) => format!(
            "{}[ {} ]",
            get_expression_rep(&*p.base, depth),
            get_expression_rep(&*p.index, depth)
        ),
        Expression::Property(p) => get_property_rep(p, depth),
        Expression::Ternary(p) => format!(
            "{} ? {} : {}",
            get_expression_rep(&*p.condition, depth),
            get_expression_rep(&*p.true_value, depth),
            get_expression_rep(&*p.false_value, depth)
        ),
        Expression::Binary(p) => get_binary_rep(p, depth),
        Expression::Prefix(_) => todo!(),
        Expression::Postfix(_) => todo!(),
        Expression::Comma(_) => todo!(),
        Expression::Table(_) => todo!(),
        Expression::Class(_) => todo!(),
        Expression::Array(_) => todo!(),
        Expression::Function(p) => get_function_rep(p, depth),
        Expression::Call(p) => get_call_rep(p, depth),
        Expression::Delegate(p) => format!(
            "delegate {} : {}",
            get_expression_rep(&*p.parent, depth),
            get_expression_rep(&*p.value, depth)
        ),
        Expression::Vector(p) => get_vector_rep(p, depth),
        Expression::Expect(p) => {
            let padding = " "; // TODO: read from config
            format!(
                "expect {}({padding}{}{padding})",
                get_typed_type_rep(&p.ty, depth),
                get_expression_rep(&*p.value, depth)
            )
        }
    }
}
