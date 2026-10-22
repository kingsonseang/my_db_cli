mod backend;
mod bootstrap;
mod creds;
mod db;
mod docker;
mod endpoint;
mod error;
mod mock;
mod pg;
mod report;
mod scope;

use backend::Backend;
use bootstrap::ensure;
use clap::{Args, Parser, Subcommand};
use db::Database;
use error::DbError;
use mock::MockDb;
use scope::Scope;

// ---------- generic dispatch ----------
// TS: function printStatus(db: Database)
// Rust: the bound `D: Database` means "any type implementing Database".
// Monomorphization: the compiler generates a copy per concrete type.

fn print_status<D: Database>(db: &D) -> Result<(), DbError> {
    println!("{} on port {}", db.kind(), db.kind().default_port());
    for name in db.list()? {
        println!("  - {name}");
    }
    Ok(())
}

// ---------- CLI ----------

#[derive(Parser)]
#[command(
    name = "my_db_cli",
    version,
    about = "Experimental dev database lifecycle tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show what backends are reachable
    Status(StatusArgs),
    /// Create databases and collections inside the active scope
    Setup(SetupArgs),
    /// Drop data but keep structure
    Reset(ResetArgs),
    /// Drop everything in the active scope
    Teardown(TeardownArgs),
    /// Check docker, credentials, and connectivity
    Doctor,
}

#[derive(Args)]
struct StatusArgs {
    /// Check only this backend (omit to check all)
    #[arg(long, value_enum)]
    backend: Option<Backend>,
}

#[derive(Args)]
struct SetupArgs {
    /// Which backend to set up
    #[arg(long, value_enum)]
    backend: Backend,

    /// Prefix for all managed database names
    #[arg(long, default_value = "play_")]
    prefix: String,

    /// Start the backend if it is not running
    #[arg(long)]
    up: bool,
}

#[derive(Args)]
struct ResetArgs {
    /// Which backend to reset
    #[arg(long, value_enum)]
    backend: Backend,

    #[arg(long, default_value = "play_")]
    prefix: String,

    /// Skip the confirmation prompt
    #[arg(long)]
    yes: bool,
}

#[derive(Args)]
struct TeardownArgs {
    /// Which backend to tear down
    #[arg(long, value_enum)]
    backend: Backend,

    #[arg(long, default_value = "play_")]
    prefix: String,

    /// Skip the confirmation prompt
    #[arg(long)]
    yes: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli.command) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

// The real logic lives here so it can use `?`.
// `?` means: "if this is Err, stop and hand the error to my caller."
fn run(command: Command) -> Result<(), DbError> {
    match command {
        Command::Status(args) => match args.backend {
            Some(b) => {
                let db = MockDb::new(b);
                match db.probe()? {
                    true => print_status(&db)?,
                    false => println!("{b} is not reachable on port {}", b.default_port()),
                }
            }
            None => {
                for b in [Backend::Mongo, Backend::Postgres] {
                    let db = MockDb::new(b);
                    match db.probe()? {
                        true => print_status(&db)?,
                        false => println!("{b} is not reachable on port {}", b.default_port()),
                    }
                }
            }
        },
        Command::Setup(args) => {
            let scope = Scope::parse(&args.prefix)?;
            let mut db = MockDb::new(args.backend);
            let report = db.setup(&scope)?;
            println!(
                "setup {} -> touched: {:?}, skipped: {:?}",
                report.backend, report.touched, report.skipped
            );
            // `contains` is the default trait method. MockDb never wrote it.
            println!(
                "contains {}mydb: {}",
                scope.prefix(),
                db.contains(&format!("{}mydb", scope.prefix()))?
            );
        }
        Command::Reset(args) => {
            let mut db = MockDb::new(args.backend);
            let scope = Scope::parse(&args.prefix)?;
            let report = db.setup(&scope)?;
            db.reset(&scope)?;
            println!(
                "setup {} -> touched: {:?}, skipped: {:?}",
                report.backend, report.touched, report.skipped
            );
        }
        Command::Teardown(args) => {
            let mut db = MockDb::new(args.backend);
            let scope = Scope::parse(&args.prefix)?;
            let report = db.setup(&scope)?;
            db.teardown(&scope)?;
            println!(
                "setup {} -> touched: {:?}, skipped: {:?}",
                report.backend, report.touched, report.skipped
            );
        }
        Command::Doctor => {
            for b in [Backend::Mongo, Backend::Postgres] {
                match ensure(b) {
                    Ok(ep) => println!("{b} is reachable on {}:{}", ep.host, ep.port),
                    Err(e) => println!("{b} MISSING: {e}"),
                }
            }
        }
    }
    Ok(())
}
