mod binary_rep;
mod function_rep;
mod global_rep;
mod literal_rep;
mod property_rep;
mod try_rep;
mod type_rep;
mod var_rep;
mod yields_rep;

use binary_rep::get_binary_rep;
use function_rep::{get_function_definition_rep, get_function_rep};
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
        println!("{}", get_statement_rep(&statement.ty))
    }
}

fn get_statement_rep(statement: &StatementType) -> String {
    let rep: String = match &statement {
        StatementType::Empty(_) => todo!(),
        StatementType::Block(_) => todo!(),
        StatementType::If(_) => todo!(),
        StatementType::While(_) => todo!(),
        StatementType::DoWhile(_) => todo!(),
        StatementType::Switch(_) => todo!(),
        StatementType::For(_) => todo!(),
        StatementType::Foreach(_) => todo!(),
        StatementType::Break(_) => String::from("break"),
        StatementType::Continue(_) => String::from("continue"),
        StatementType::Return(p) => get_return_rep(p),
        StatementType::Yield(p) => get_yield_rep(p),
        StatementType::VarDefinition(p) => get_var_definition_list_rep(p),
        StatementType::ConstructorDefinition(_) => todo!(),
        StatementType::FunctionDefinition(p) => get_function_definition_rep(p),
        StatementType::ClassDefinition(_) => todo!(),
        StatementType::TryCatch(_) => todo!(),
        StatementType::Throw(p) => throw_rep(p),
        StatementType::Const(p) => get_const_rep(p),
        StatementType::EnumDefinition(_) => todo!(),
        StatementType::Expression(_) => todo!(),
        StatementType::Thread(_) => todo!(),
        StatementType::DelayThread(p) => get_delaythread_rep(p),
        StatementType::WaitThread(_) => String::from("waitthread"),
        StatementType::WaitThreadSolo(_) => String::from("waitthreadsolo"),
        StatementType::Wait(p) => format!("wait {}", get_expression_rep(&*p.value)),
        StatementType::StructDefinition(_) => todo!(),
        StatementType::TypeDefinition(p) => get_typedef_rep(p),
        StatementType::Global(p) => get_global_rep(p),
        StatementType::GlobalizeAllFunctions(_) => String::from("globalize_all_functions"),
        StatementType::Untyped(_) => String::from("untyped"),
    };
    format!("{rep}")
}

// EXPRESSIONS

fn get_expression_rep(expression: &Expression) -> String {
    match expression {
        Expression::Parens(p) => format!("( {} )", get_expression_rep(&*p.value)),
        Expression::Literal(p) => get_literal_rep(p),
        Expression::Var(p) => String::from(p.name.value),
        Expression::RootVar(p) => format!("::{}", p.name.value),
        Expression::Index(p) => format!(
            "{}[ {} ]",
            get_expression_rep(&*p.base),
            get_expression_rep(&*p.index)
        ),
        Expression::Property(p) => get_property_rep(p),
        Expression::Ternary(p) => format!(
            "{} ? {} : {}",
            get_expression_rep(&*p.condition),
            get_expression_rep(&*p.true_value),
            get_expression_rep(&*p.false_value)
        ),
        Expression::Binary(p) => get_binary_rep(p),
        Expression::Prefix(_) => todo!(),
        Expression::Postfix(_) => todo!(),
        Expression::Comma(_) => todo!(),
        Expression::Table(_) => todo!(),
        Expression::Class(_) => todo!(),
        Expression::Array(_) => todo!(),
        Expression::Function(p) => get_function_rep(p),
        Expression::Call(_) => todo!(),
        Expression::Delegate(p) => format!(
            "delegate {} : {}",
            get_expression_rep(&*p.parent),
            get_expression_rep(&*p.value)
        ),
        Expression::Vector(p) => get_vector_rep(p),
        Expression::Expect(p) => {
            let padding = " "; // TODO: read from config
            format!(
                "expect {}({padding}{}{padding})",
                get_typed_type_rep(&p.ty),
                get_expression_rep(&*p.value)
            )
        }
    }
}
