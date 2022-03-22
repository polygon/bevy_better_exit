use bevy::log::info;
use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_better_exit::{BetterExitPlugin, ExitEvent, ExitListener};
use std::time::Duration;

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(BetterExitPlugin::new(Some(5.0)))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut exit: EventWriter<ExitEvent>) {
    info!("Creating unhandled exit handler");
    commands
        .spawn()
        .insert(ExitListener::new(Some("Unhandled")));
    info!("Sending ExitEvent");
    exit.send(ExitEvent);
}
