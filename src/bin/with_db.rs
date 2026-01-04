use std::{
    env,
    process::{Command, ExitCode},
};

fn main() -> ExitCode {
    let database_url = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "missing required env var: DATABASE_URL"
            );
            eprintln!(
                "hint: run via `cargo run --bin with_db -- <cmd> ...` so .cargo/config.toml is applied"
            );
            return ExitCode::from(2);
        }
    };

    let mut args = env::args_os();
    let _bin = args.next(); // argv[0]

    let cmd = match args.next() {
        Some(c) => c,
        None => {
            eprintln!(
                "usage: cargo run --bin with_db -- <command> [args...]"
            );
            eprintln!(
                "example: cargo run --bin with_db -- sqlx migrate run --source crates/infra/migrations"
            );
            return ExitCode::from(2);
        }
    };

    let status = Command::new(cmd)
        .args(args)
        .env("DATABASE_URL", database_url)
        .status();

    match status {
        Ok(s) if s.success() => ExitCode::SUCCESS,
        Ok(s) => {
            ExitCode::from(s.code().unwrap_or(1) as u8)
        }
        Err(e) => {
            eprintln!("failed to exec command: {e}");
            ExitCode::from(127)
        }
    }
}
