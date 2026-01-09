extern crate ratatui;
use flexi_logger::{AdaptiveFormat, FileSpec, Logger, WriteMode, detailed_format};
use ratatui::{DefaultTerminal, Frame};
use sha_scaffolder::{Scaffolder};

fn main() -> color_eyre::Result<()> {
    let _logger = Logger::try_with_str("info")?
        .adaptive_format_for_stdout(AdaptiveFormat::Default)
        .format_for_files(detailed_format)
        .log_to_file(FileSpec::default().directory("logs"))
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    let main_config_path: &str = "/examples/elysia/sha.toml";
    Scaffolder::new(main_config_path);
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
}
