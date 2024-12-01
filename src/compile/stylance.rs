use crate::{
    config::StylanceConfig,
    ext::{
        anyhow::{Context, Result},
        sync::{wait_piped_interruptible, CommandResult, OutputExt},
        Exe,
    },
    logger::GRAY,
    signal::{Interrupt, Outcome},
};
use tokio::process::Command;

pub async fn compile_stylance(config: &StylanceConfig) -> Result<Outcome<String>> {
    let args = vec![
        config.package_dir.as_str(),
        "--output-file",
        config.tmp_file.as_str(),
    ];

    let exe = Exe::Stylance.get().await.dot()?;

    let mut cmd = Command::new(exe);
    cmd.args(&args);

    log::trace!(
        "Stylance running {}",
        GRAY.paint(format!("stylance {}", args.join(" ")))
    );

    match wait_piped_interruptible("Stylance", cmd, Interrupt::subscribe_any()).await? {
        CommandResult::Success(_) => std::fs::read_to_string(&config.tmp_file)
            .with_context(|| format!("failed to read stylance output {:?}", config.tmp_file))
            .map(Outcome::Success),
        CommandResult::Interrupted => Ok(Outcome::Stopped),
        CommandResult::Failure(output) => {
            log::warn!("Stylance failed with:");
            println!("{}", output.stderr());
            Ok(Outcome::Failed)
        }
    }
}
