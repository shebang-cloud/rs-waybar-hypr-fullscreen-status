use hyprland::event_listener::EventListenerMutable;

use crate::args::Args;
use crate::prelude::*;
use crate::query;

/// Listen to the fullscreen status of a monitor.
///
/// Output the current status and whenever status changes.
///
/// # Errors
/// Propagate any `HyprError`
pub fn listen_monitor_fullsecreen_status(args: Args) -> Result<()> {
    // Print the current value:
    if let Ok(status) = query::monitor_fullscreen_status(&args) {
        println!("{status}");
    }

    let mut event_listener = EventListenerMutable::new();
    event_listener.add_fullscreen_state_change_handler(move |_, state| {
        if state.active_monitor == args.monitor_name {
            if let Ok(status) = query::monitor_fullscreen_status(&args) {
                println!("{status}");
            }
        }
    });

    // Listen to changes, blocks!
    event_listener.start_listener()?;

    Ok(())
}
