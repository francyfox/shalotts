extern crate ratatui;
use flexi_logger::{FileSpec, Logger, WriteMode};
use ratatui::{DefaultTerminal, Frame};
use sha_validator::validate;

fn main() -> color_eyre::Result<()> {
    let _logger = Logger::try_with_str("info")?
         .log_to_file(FileSpec::default().directory("logs"))
         .duplicate_to_stderr(flexi_logger::Duplicate::All)
         .write_mode(WriteMode::BufferAndFlush)
         .start()?;
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
    let main_config_path: &str = "/examples/elysia/sha.toml";
    match validate(main_config_path) {
        Ok(config) => println!("Config valid: {:?}", config),
        Err(e) => eprintln!("Validation error: {}", e),
    }
}
