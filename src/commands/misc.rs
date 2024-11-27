use crate::{
    global::*,
    utils::*
};
use sysinfo::System;

#[poise::command(prefix_command, slash_command)]
pub async fn sysinfo(ctx: Context<'_>) -> Result<(), Error> {
    // System info
    let system = System::new_all();

    let name = format!(
        "Name: `{}`",
        System::name().unwrap_or("N/A".to_string())
    );
    let kernel = format!(
        "Kernel: `{}`",
        System::kernel_version().unwrap_or("N/A".to_string())
    );
    let version = format!(
        "Version: `{}`",
        System::os_version().unwrap_or("N/A".to_string())
    );
    let hostname = format!(
        "Hostname: `{}`",
        System::host_name().unwrap_or("N/A".to_string())
    );
    let uptime = format!(
        "Uptime: `{}`",
        pretty_seconds(System::uptime())
    );
    let cpus = format!(
        "CPUs: `{}`",
        system.cpus().len()
    );
    let memory = format!(
        "Memory: `{}` / `{}`",
        pretty_bytes(system.used_memory()),
        pretty_bytes(system.total_memory())
    );
    let swap = format!(
        "Swap: `{}` / `{}`",
        pretty_bytes(system.used_swap()),
        pretty_bytes(system.total_swap())
    );

    // Package info
    let bot = format!("markov-chains-bot: `{}`", env!("CARGO_PKG_VERSION"));
    let markov = format!("markov-chains: `{}`", MARKOV_CHAINS_VERSION);

    ctx.say(format!(
        "## System\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\n## Package\n{}\n{}",
        name, kernel, version, hostname, uptime, cpus, memory, swap, bot, markov
    )).await?;

    Ok(())
}
