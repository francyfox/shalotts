use flexi_logger::{Logger, detailed_format};
use color_eyre::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use sha_scaffolder::Scaffolder;
use std::time::Duration;

fn main() -> Result<()> {
    color_eyre::install()?;
    let _logger = Logger::try_with_str("warn")?
        .format_for_files(detailed_format)
        .log_to_file(flexi_logger::FileSpec::default().directory("logs"))
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .start()?;

    print_banner();

    let main_config_path = "examples/elysia/sha.toml";
    let output_path = "./test-app";

    println!("\n{} {}", "⚡".bright_yellow(), "Starting scaffolding...".bright_white().bold());
    println!("  {} {}", "→".bright_cyan(), format!("Config: {}", main_config_path).dimmed());
    println!("  {} {}", "→".bright_cyan(), format!("Output: {}", output_path).dimmed());
    println!();

    // Create spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    spinner.enable_steady_tick(Duration::from_millis(80));

    spinner.set_message("Validating configuration...".to_string());
    let scaffolder = Scaffolder::new(main_config_path);
    spinner.finish_and_clear();
    println!("  {} Configuration validated", "✓".green());

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_message("Generating files...".to_string());

    match scaffolder.generate(main_config_path, output_path) {
        Ok(_) => {
            spinner.finish_and_clear();
            println!("  {} Files generated", "✓".green());
            print_success(output_path);
        }
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("\n  {} {}", "✗".red(), format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_banner() {
    let banner = format!(
        r#"
  {}  {}  v{}
        "#,
        "SHALOTTS".bright_magenta().bold(),
        "⚡".bright_yellow(),
        env!("CARGO_PKG_VERSION").dimmed()
    );
    println!("{}", banner);
}

fn print_success(output_path: &str) {
    println!();
    println!("  {} {}", "✓".green().bold(), "Done! Your project is ready.".bright_white().bold());
    println!();
    println!("  {} Next steps:", "→".bright_cyan());
    println!("    {} cd {}", "$".dimmed(), output_path.bright_cyan());
    println!("    {} npm install", "$".dimmed());
    println!("    {} npm run dev", "$".dimmed());
    println!();
}
