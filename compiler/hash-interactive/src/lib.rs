//! Main module for Hash interactive mode.

mod command;

use command::InteractiveCommand;
use hash_pipeline::{
    settings::CompilerJobParams,
    sources::InteractiveBlock,
    traits::{Desugar, Parser, SemanticPass, Tc, VirtualMachine},
    Compiler, CompilerState,
};
use hash_reporting::errors::{CompilerError, InteractiveCommandError};
use hash_source::SourceId;
use rustyline::{error::ReadlineError, Editor};
use std::{env, process::exit};

type CompilerResult<T> = Result<T, CompilerError>;

/// Interactive backend version
pub const VERSION: &str = env!("EXECUTABLE_VERSION");

/// Utility to print the version of the current interactive backend
#[inline(always)]
pub fn print_version() {
    println!("Version {}", VERSION);
}

/// Function that is called on a graceful interpreter exit
pub fn goodbye() {
    println!("Goodbye!");
    exit(0)
}

/// Function that initialises the interactive mode. Setup all the resources
/// required to perform execution of provided statements and then initiate the
/// REPL.
pub fn init<'c, 'pool, P, D, S, C, V>(
    mut compiler: Compiler<'pool, P, D, S, C, V>,
    mut compiler_state: CompilerState<'c, 'pool, D, S, C, V>,
) -> CompilerResult<()>
where
    'pool: 'c,
    P: Parser<'pool>,
    D: Desugar<'pool>,
    S: SemanticPass<'pool>,
    C: Tc<'c>,
    V: VirtualMachine<'c>,
{
    // Display the version on start-up
    print_version();

    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline(">>> ");

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                compiler_state = execute(line.as_str(), &mut compiler, compiler_state);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting!");
                break;
            }
            Err(err) => {
                return Err(InteractiveCommandError::InternalError(format!(
                    "Unexpected error: {}",
                    err
                ))
                .into());
            }
        }
    }

    Ok(())
}

/// Function to process a single line of input from the REPL instance.
fn execute<'c, 'pool, P, D, S, C, V>(
    input: &str,
    compiler: &mut Compiler<'pool, P, D, S, C, V>,
    mut compiler_state: CompilerState<'c, 'pool, D, S, C, V>,
) -> CompilerState<'c, 'pool, D, S, C, V>
where
    'pool: 'c,
    P: Parser<'pool>,
    D: Desugar<'pool>,
    S: SemanticPass<'pool>,
    C: Tc<'c>,
    V: VirtualMachine<'c>,
{
    if input.is_empty() {
        return compiler_state;
    }

    let command = InteractiveCommand::from(input);

    match command {
        Ok(InteractiveCommand::Quit) => goodbye(),
        Ok(InteractiveCommand::Clear) => {
            // check if this is either a unix/windows system and then execute
            // the appropriate clearing command
            if cfg!(target_os = "windows") {
                std::process::Command::new("cls").status().unwrap();
            } else {
                std::process::Command::new("clear").status().unwrap();
            }
        }
        Ok(InteractiveCommand::Version) => print_version(),
        Ok(
            ref inner @ (InteractiveCommand::Type(expr)
            | InteractiveCommand::Display(expr)
            | InteractiveCommand::Code(expr)),
        ) => {
            // Add the interactive block to the state
            let interactive_id = compiler_state
                .workspace
                .add_interactive_block(expr.to_string(), InteractiveBlock::new());

            // Compute the mode of the job based on provided arguments via the interactive
            // command
            let settings: CompilerJobParams = inner.into();

            // We don't want the old diagnostics
            // @@Refactor: we don't want to leak the diagnostics here..
            compiler_state.diagnostics.clear();
            let new_state =
                compiler.run(SourceId::Interactive(interactive_id), compiler_state, settings);
            return new_state;
        }
        Err(e) => CompilerError::from(e).report(),
    }

    compiler_state
}
