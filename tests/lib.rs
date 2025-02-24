#[macro_use]
mod test;

mod assert_stdout;
mod assert_success;
mod byte_order_mark;
mod changelog;
mod choose;
mod command;
mod common;
mod completions;
mod conditional;
mod delimiters;
mod dotenv;
mod edit;
mod equals;
mod error_messages;
mod evaluate;
mod examples;
mod export;
mod fmt;
mod functions;
mod init;
mod interrupts;
mod invocation_directory;
mod json;
mod line_prefixes;
mod misc;
mod positional_arguments;
mod quiet;
mod quote;
mod readme;
mod regexes;
mod search;
mod shebang;
mod shell;
mod show;
mod string;
mod sublime_syntax;
mod subsequents;
mod tempdir;
mod undefined_variables;
#[cfg(target_family = "windows")]
mod windows_powershell;
mod working_directory;
