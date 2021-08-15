use clap::{crate_version,AppSettings, Clap};

/// A simple Pomodoro timer for the command line
#[derive(Clap)]
#[clap(version = crate_version!())]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommands,
}

#[derive(Clap)]
enum SubCommands {
    Pomodoro(Pomodoro),
    Break(Break),
}

/// Work with a pomodoro
#[derive(Clap)]
struct Pomodoro {
    #[clap(subcommand)]
    subcmd: PomodoroCommands,
}

#[derive(Clap)]
enum PomodoroCommands {
    Start(StartPomodoro),
    Interrupt(InterruptPomodoro),
    Cancel(CancelPomodoro),
    Finish(FinishPomodoro),
}

/// Start a Pomodoro
#[derive(Clap)]
struct StartPomodoro {
    /// How many minutes this Pomodoro should last
    #[clap(short, long, required(false), default_value("25"), takes_value(true), value_name("DURATION"))]
    duration: i8
}

/// Finish the active Pomodoro
#[derive(Clap)]
struct FinishPomodoro {
}

/// Interrupt the active Pomodoro
#[derive(Clap)]
struct InterruptPomodoro {
}

/// Cancel the active Pomodoro
#[derive(Clap)]
struct CancelPomodoro {
}

/// Work with a break
#[derive(Clap)]
struct Break {
    #[clap(subcommand)]
    subcmd: BreakCommands,
}

#[derive(Clap)]
enum BreakCommands {
    Start(StartBreak),
    Finish(FinishBreak),
}

/// Start a Break
#[derive(Clap)]
struct StartBreak {
    /// How many minutes this break should last
    #[clap(short, long, required(false), default_value("5"), takes_value(true), value_name("DURATION"))]
    duration: i8
}

/// Finish the active Break
#[derive(Clap)]
struct FinishBreak {
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommands::Pomodoro(pomodoro_options) => {
            match pomodoro_options.subcmd {
              PomodoroCommands::Start(start_options) => {
                println!("Starting a new Pomodoro that will last for {} minutes", start_options.duration);
              }
              PomodoroCommands::Interrupt(_) => {
                println!("Marking the active Pomodoro as interrupted");
              }
              PomodoroCommands::Cancel(_) => {
                println!("Cancelling the active Pomodoro");
              }
              PomodoroCommands::Finish(_) => {
                println!("Finishing the active Pomodoro");
              }
            }
        }
        SubCommands::Break(break_options) => {
            match break_options.subcmd {
              BreakCommands::Start(start_options) => {
                println!("Starting a new break that will last for {} minutes", start_options.duration);
              }
              BreakCommands::Finish(_) => {
                println!("Finishing the active Break");
              }
            }
        }
    }
}
