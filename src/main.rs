use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::Context;
use gag::Gag;
use inquire::{Confirm, Select};
use sam_rs::get_achievements;
use serde::Serialize;
use std::fmt;
use std::io::Write;
use std::process::exit;

use steamworks::Client;

/// Steam Achievement Manager, now in Rust!
#[derive(Debug, Parser)]
#[command(name = "sam")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    app_id: u32,
}

#[derive(Debug, Clone, ValueEnum, Serialize)]
enum ListFormat {
    Raw,
    Ssv,
    Csv,
    Json,
}

impl fmt::Display for ListFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Raw => write!(f, "raw"),
            Self::Ssv => write!(f, "ssv"),
            Self::Csv => write!(f, "csv"),
            Self::Json => write!(f, "json"),
        }
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List achievements
    #[command(name = "list")]
    ListAchievements {
        #[arg(short, long, default_value_t = ListFormat::Raw)]
        format: ListFormat,
    },
    /// Set provided achievement
    #[command(name = "set")]
    SetAchievement {
        /// Name for achievement used internally by Steam
        internal_name: String,
    },
    /// Clear provided achievement
    #[command(name = "clear")]
    ClearAchievement {
        /// Name for achievement used internally by Steam
        internal_name: String,
    },
    /// Get achievement status
    #[command(arg_required_else_help = true, name = "get")]
    GetAchievement {
        /// Name for achievement used internally by Steam
        internal_name: String,
    },
    /// Open an interactive achievement picker
    Interactive {},
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let command = args.command;

    let client = {
        let _gag_stderr = Gag::stderr().context("Failed to install gag during client init")?;
        Client::init_app(args.app_id)?
    };

    match command {
        Commands::ListAchievements { format } => {
            let achievement_data = sam_rs::get_achievements(&client);

            match format {
                ListFormat::Raw | ListFormat::Ssv => print_as_ssv(achievement_data)?,
                ListFormat::Csv => print_as_csv(achievement_data)?,
                ListFormat::Json => print_as_json(achievement_data)?,
            }
        }
        Commands::SetAchievement { internal_name } => {
            sam_rs::set_achievement(&client, &internal_name)?;
        }
        Commands::ClearAchievement { internal_name } => {
            sam_rs::clear_achievement(&client, &internal_name)?;
        }
        Commands::GetAchievement { internal_name } => {
            println!("{}", sam_rs::get_achievement(&client, &internal_name)?);
        }
        Commands::Interactive {} => {
            println!("Current app id: {}", client.utils().app_id().0);
            println!("Signed in as {}", client.friends().name());
            let ans = Confirm::new("Does this information look correct?")
                .with_default(false)
                .prompt()?;

            if !ans {
                exit(0);
            }

            let achievements = get_achievements(&client).collect();

            let selected_achievement =
                Select::new("Select an achievement to manage", achievements).prompt()?;

            let selected_action =
                Select::new("What would you like to do?", vec!["Set", "Clear"]).prompt()?;

            match selected_action {
                "Set" => sam_rs::set_achievement(&client, &selected_achievement.internal_name)?,
                "Clear" => sam_rs::clear_achievement(&client, &selected_achievement.internal_name)?,
                _ => (),
            }
        }
    }

    Ok(())
}

fn print_as_json(achievement_data: impl Iterator<Item = sam_rs::Achievement>) -> Result<()> {
    println!(
        "{}",
        serde_json::ser::to_string_pretty::<Vec<sam_rs::Achievement>>(&achievement_data.collect())?
    );
    Ok(())
}

fn print_as_csv(achievement_data: impl Iterator<Item = sam_rs::Achievement>) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for ach in achievement_data {
        wtr.serialize(ach)?;
    }
    println!("{}", String::from_utf8(wtr.into_inner()?)?);
    Ok(())
}

fn print_as_ssv(achievement_data: impl Iterator<Item = sam_rs::Achievement>) -> Result<()> {
    let stdout = std::io::stdout();
    let mut output = tabwriter::TabWriter::new(stdout.lock());
    writeln!(
        output,
        "internal_name\tdisplay_name\tdescription\tis_hidden\tuser_has_obtained"
    )?;
    for achievement in achievement_data {
        writeln!(
            output,
            "{}\t{}\t{}\t{}\t{}",
            achievement.internal_name,
            achievement.display_name,
            achievement.description,
            achievement.is_hidden,
            achievement.user_has_obtained,
        )?;
    }
    output.flush()?;
    Ok(())
}
