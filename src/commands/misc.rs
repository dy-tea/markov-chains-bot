use crate::{
    global::*,
    utils::*
};
use sysinfo::System;

/// Show system information
#[poise::command(prefix_command, slash_command)]
pub async fn sysinfo(ctx: Context<'_>) -> Result<(), Error> {
    // System info
    let system = System::new_all();

    let name = format!(
        "**Name:**\t`{}`",
        System::name().unwrap_or("N/A".to_string())
    );
    let kernel = format!(
        "**Kernel:**\t`{}`",
        System::kernel_version().unwrap_or("N/A".to_string())
    );
    let version = format!(
        "**Version:**\t`{}`",
        System::os_version().unwrap_or("N/A".to_string())
    );
    let hostname = format!(
        "**Hostname:**\t`{}`",
        System::host_name().unwrap_or("N/A".to_string())
    );
    let uptime = format!(
        "**Uptime:**\t`{}`",
        pretty_seconds(System::uptime())
    );
    let cpus = format!(
        "**CPUs:**\t`{}`",
        system.cpus().len()
    );
    let memory = format!(
        "**Memory:**\t`{}`\t**/**\t`{}`",
        pretty_bytes(system.used_memory()),
        pretty_bytes(system.total_memory())
    );
    let swap = format!(
        "**Swap:**\t`{}`\t**/**\t`{}`",
        pretty_bytes(system.used_swap()),
        pretty_bytes(system.total_swap())
    );

    // Package info
    let bot = format!("**markov-chains-bot:**\t`{}`", env!("CARGO_PKG_VERSION"));
    let markov = format!("**markov-chains:**\t`{}`", MARKOV_CHAINS_VERSION);

    ctx.say(format!(
        "## System\n- {}\n- {}\n- {}\n- {}\n- {}\n- {}\n- {}\n- {}\n## Package\n- {}\n- {}",
        name, kernel, version, hostname, uptime, cpus, memory, swap, bot, markov
    )).await?;

    Ok(())
}
