use clap::{CommandFactory, Parser};
use pe2_cli::args::Args;
use pe2_core::validation;

#[test]
fn cli_args_have_version_and_about() {
    let cmd = Args::command();
    assert_eq!(cmd.get_version().unwrap(), env!("CARGO_PKG_VERSION"));
    assert!(cmd.get_about().is_some());
}

#[test]
fn resolve_iterations_defaults_to_auto_difficulty() {
    let args = Args::try_parse_from(["pe2", "this is a long enough prompt"]).unwrap();
    assert!(args.auto_difficulty);
    assert!(args.iterations.is_none());
}

#[test]
fn explicit_iterations_disables_auto_difficulty_default() {
    let args = Args::try_parse_from(["pe2", "--iterations", "3", "this is a long enough prompt"])
        .unwrap();
    assert_eq!(args.iterations, Some(3));
}

#[test]
fn pipeline_validation_rejects_short_prompt() {
    assert!(validation::validate_prompt("short").is_some());
}

#[test]
fn slash_commands_parse_known_aliases() {
    assert_eq!(validation::parse_slash_command("/c"), Some("/c"));
    assert_eq!(validation::parse_slash_command("/stats"), Some("/stats"));
}
