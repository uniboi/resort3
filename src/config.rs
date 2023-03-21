use std::{error::Error, path::PathBuf};
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
	// general settings
    pub semicolons: Option<bool>,
    pub trim_float: Option<bool>,

    // pool settings
    pub padding: Option<bool>,
    pub gap: Option<bool>,
    pub inline: Option<bool>,
    pub inline_block: Option<bool>,

    // pool overrides
    pub delaythread_padding: Option<bool>,
    pub delaythread_gap: Option<bool>,
    pub array_oneliner_max: Option<usize>,
    pub array_oneliner_definition_padding: Option<bool>,
    pub expect_padding: Option<bool>,
    pub vector_padding: Option<bool>,
    pub functionref_oneliner_args_max: Option<usize>,
    pub non_generic_type_padding: Option<bool>,
    pub table_oneliner_max: Option<usize>,
    pub if_gap: Option<bool>,
    pub if_padding: Option<bool>,
    pub if_inline: Option<bool>,
    pub if_inline_block: Option<bool>,
    pub for_gap: Option<bool>,
    pub for_padding: Option<bool>,
    pub for_inline: Option<bool>,
    pub for_inline_block: Option<bool>,
    pub foreach_gap: Option<bool>,
    pub foreach_padding: Option<bool>,
    pub foreach_inline: Option<bool>,
    pub foreach_inline_block: Option<bool>,
    pub while_gap: Option<bool>,
    pub while_padding: Option<bool>,
    pub while_inline: Option<bool>,
    pub while_inline_block: Option<bool>,
    pub do_while_inline: Option<bool>,
    pub do_while_inline_block: Option<bool>,
}

pub struct Config {
    /// Add semicolons to statements
    pub semicolons: bool,
    /// add padding to the brackets of `delaythread`
    pub delaythread_padding: bool,
    /// add a gap between `delaythread` and the timer brackets
    pub delaythread_gap: bool,
    /// defines the number of items in an array definition that will be represented in a single line defintion
    pub array_oneliner_max: usize,
    /// add padding in oneliner array definitions
    pub array_oneliner_definition_padding: bool,
    /// add padding in the var brackets
    pub expect_padding: bool,
    /// trim the leading 0 of a float and start at the dot, if possible
    pub trim_float: bool,
    /// add padding between between the open and close tokens of a vector
    pub vector_padding: bool,
    /// defines the number of args a functionref takes before it will be forced to be multiline
    pub functionref_oneliner_args_max: usize,
    /// add padding to generics holding non-generic types. e.g. `table<string, int>`
    pub non_generic_type_padding: bool,
    /// defines the number of items in a table definition that will be represented in a single line definition
    pub table_oneliner_max: usize,
    /// add a gap between `if` and the condition
    pub if_gap: bool,
    /// add padding in the condition
    pub if_padding: bool,
    /// inline non block statements
    pub if_inline: bool,
    /// start the following block in the same line
    pub if_inline_block: bool,
    /// add a gap between `for` and the condition
    pub for_gap: bool,
    /// add padding in the condition
    pub for_padding: bool,
    /// single inline statement
    pub for_inline: bool,
    /// start the following block in the same line
    pub for_inline_block: bool,
    /// add a gap between `foreach` and the condition
    pub foreach_gap: bool,
    /// add padding in the condition
    pub foreach_padding: bool,
    /// add a gap between `while` and the condition
    pub foreach_inline: bool,
    /// start the following block in the same line
    pub foreach_inline_block: bool,
    /// add a gap after the while token
    pub while_gap: bool,
    /// add padding in the condition
    pub while_padding: bool,
    /// inline single statement
    pub while_inline: bool,
    /// start the following block in the same line
    pub while_inline_block: bool,
    /// inline single statement
    pub do_while_inline: bool,
    /// start the following block in the same line
    pub do_while_inline_block: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            semicolons: false,
            delaythread_padding: true,
            delaythread_gap: false,
            array_oneliner_max: 5,
            array_oneliner_definition_padding: true,
            expect_padding: true,
            trim_float: false,
            vector_padding: true,
            functionref_oneliner_args_max: 5,
            non_generic_type_padding: false,
            table_oneliner_max: 3,
            if_gap: true,
            if_padding: true,
            if_inline: false,
            if_inline_block: false,
            for_gap: true,
            for_padding: true,
            for_inline: false,
            for_inline_block: false,
            foreach_gap: true,
            foreach_padding: true,
            foreach_inline: false,
            foreach_inline_block: false,
            while_gap: true,
            while_padding: true,
            while_inline: false,
            while_inline_block: false,
            do_while_inline: false,
            do_while_inline_block: false,
        }
    }

    pub fn from_path(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let default: Config = Config::new();
        let p: ConfigFile = serde_json::from_reader(reader)?;

        Ok(Self {
            semicolons: p.semicolons.unwrap_or(default.semicolons),
            delaythread_padding: p
                .delaythread_padding
                .unwrap_or(p.padding.unwrap_or(default.delaythread_padding)),
            delaythread_gap: p
                .delaythread_gap
                .unwrap_or(p.gap.unwrap_or(default.delaythread_gap)),
            array_oneliner_max: p.array_oneliner_max.unwrap_or(default.array_oneliner_max),
            array_oneliner_definition_padding: p.array_oneliner_definition_padding.unwrap_or(
                p.padding
                    .unwrap_or(default.array_oneliner_definition_padding),
            ),
            expect_padding: p
                .expect_padding
                .unwrap_or(p.padding.unwrap_or(default.expect_padding)),
            trim_float: p.trim_float.unwrap_or(default.trim_float),
            vector_padding: p
                .vector_padding
                .unwrap_or(p.padding.unwrap_or(default.vector_padding)),
            functionref_oneliner_args_max: p
                .functionref_oneliner_args_max
                .unwrap_or(default.functionref_oneliner_args_max),
            non_generic_type_padding: p
                .non_generic_type_padding
                .unwrap_or(p.padding.unwrap_or(default.non_generic_type_padding)),
            table_oneliner_max: p.table_oneliner_max.unwrap_or(default.table_oneliner_max),
            if_gap: p.if_gap.unwrap_or(p.gap.unwrap_or(default.if_gap)),
            if_padding: p
                .if_padding
                .unwrap_or(p.padding.unwrap_or(default.if_padding)),
            if_inline: p.if_inline.unwrap_or(p.inline.unwrap_or(default.if_inline)),
            for_gap: p.for_gap.unwrap_or(p.gap.unwrap_or(default.for_gap)),
            for_padding: p
                .for_padding
                .unwrap_or(p.padding.unwrap_or(default.for_padding)),
            for_inline: p
                .for_inline
                .unwrap_or(p.inline.unwrap_or(default.for_inline)),
            foreach_gap: p
                .foreach_gap
                .unwrap_or(p.gap.unwrap_or(default.foreach_gap)),
            foreach_padding: p
                .foreach_padding
                .unwrap_or(p.padding.unwrap_or(default.foreach_padding)),
            foreach_inline: p
                .foreach_inline
                .unwrap_or(p.inline.unwrap_or(default.foreach_inline)),
            while_gap: p.while_gap.unwrap_or(p.gap.unwrap_or(default.while_gap)),
            while_padding: p
                .while_padding
                .unwrap_or(p.padding.unwrap_or(default.while_padding)),
            while_inline: p
                .while_inline
                .unwrap_or(p.inline.unwrap_or(default.while_inline)),
            do_while_inline: p
                .do_while_inline
                .unwrap_or(p.inline.unwrap_or(default.do_while_inline)),
            if_inline_block: p
                .if_inline_block
                .unwrap_or(p.inline_block.unwrap_or(default.if_inline_block)),
            for_inline_block: p
                .for_inline_block
                .unwrap_or(p.for_inline_block.unwrap_or(default.for_inline_block)),
            foreach_inline_block: p
                .foreach_inline_block
                .unwrap_or(p.inline_block.unwrap_or(default.foreach_inline_block)),
            while_inline_block: p
                .while_inline_block
                .unwrap_or(p.inline_block.unwrap_or(default.while_inline_block)),
            do_while_inline_block: p
                .do_while_inline_block
                .unwrap_or(p.inline_block.unwrap_or(default.do_while_inline_block)),
        })
    }
}
