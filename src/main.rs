mod state;
use itertools::Itertools;
use state::EnvChangeRequest;
use std::process::exit;

pub struct CliOpts {
    show_display: bool,
    is_env_set: bool,
    mode: CliMode,
}

impl Default for CliOpts {
    fn default() -> Self {
        Self {
            is_env_set: false,
            show_display: false,
            mode: CliMode::Add,
        }
    }
}

enum CliMode {
    Add,
    After,
    Delete,
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut cli_opts = CliOpts::default();
    let mut env_options = EnvChangeRequest::default();

    handle_args(args, &mut env_options, &mut cli_opts);

    // Handle the action
    if cli_opts.show_display {
        action_display(&env_options);
    } else {
        action_eval(&env_options);
    }
}

fn handle_args(args: Vec<String>, env_options: &mut EnvChangeRequest, cli_opts: &mut CliOpts) {
    let mut index = 0;
    while index < args.len() {
        let arg = args[index].clone();

        if arg.starts_with("--env") {
            env_options.set_var(if arg.contains("=") {
                let split = split_with_sep(&arg, "=");
                if split.len() == 2 {
                    index += 1;
                    split[1]
                } else {
                    eprintln!("Parse error: Cannot parse {}", arg);
                    exit(1);
                }
            } else {
                if index != args.len() - 1 {
                    index += 2;
                    &args[index - 1]
                } else {
                    eprintln!("Parse error: missing --env=<value>");
                    exit(1);
                }
            });

            if !cli_opts.is_env_set {
                cli_opts.is_env_set = true;
            } else {
                eprintln!("Parse error: multiple --env=<value>");
                exit(1);
            }
            continue;
        }

        //TODO: Display options
        if arg.starts_with("--display") {
            cli_opts.show_display = true;
            index += 1;
            continue;
        }

        if arg == "--add" {
            cli_opts.mode = CliMode::Add;
            index += 1;
            continue;
        }

        if arg == "--after" {
            cli_opts.mode = CliMode::After;
            index += 1;
            continue;
        }

        if arg == "--delete" {
            cli_opts.mode = CliMode::Delete;
            index += 1;
            continue;
        }

        if arg.starts_with("-") {
            eprintln!("Parse error: Unknown command {}", arg);
            exit(1);
        }

        match cli_opts.mode {
            CliMode::Add => {
                env_options.push_before(&arg);
            }
            CliMode::After => {
                env_options.push_after(&arg);
            }
            CliMode::Delete => {
                env_options.push_delete(&arg);
            }
        }
        index += 1;
    }
}

fn action_display(env_options: &EnvChangeRequest) {
    let paths = env_options.process_uniq();
    println!("TODO: display action {:?}", &paths);
    action_eval(&env_options);
}

fn action_eval(env_options: &EnvChangeRequest) {
    let paths = env_options.process();
    let unique_paths: Vec<String> = paths.into_iter().unique().collect();
    let join_paths = unique_paths.join(":");
    println!(r#"export {}="{}""#, env_options.environment, &join_paths)
}

fn split_with_sep<'a>(text: &'a str, sep: &str) -> Vec<&'a str> {
    text.split(sep)
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect()
}
