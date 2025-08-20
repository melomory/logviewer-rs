use logviewer::{app::App, config, localization, logging::init_logging};

fn main() -> anyhow::Result<()> {
    init_logging();
    let config = config::loader::load_or_create_default()?;
    let mut app = App::new(config)?;

    // Test logging.
    log::info!("Started");

    // Test localization.
    println!("{}", localization::t(localization::Language::Ru, "hello"));

    app.run()
}
