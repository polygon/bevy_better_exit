use bevy::log::info;
use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_better_exit::{BetterExitPlugin, ExitEvent, ExitListener};
use std::time::Duration;

#[derive(Component)]
struct ExitProcess {
    timer: Timer,
}

#[derive(Component)]
struct ExitProcessMarker;

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 20.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(BetterExitPlugin::new(Some(5.0)))
        .add_startup_system(setup)
        .add_system(exit_listener)
        .add_system(exit_process)
        .run();
}

fn setup(mut commands: Commands, mut exit: EventWriter<ExitEvent>) {
    info!("Creating ExitListener");
    commands
        .spawn()
        .insert(ExitListener::new(Some("Process")))
        .insert(ExitProcessMarker);
    info!("Sending ExitEvent");
    exit.send(ExitEvent);
}

fn exit_listener(
    mut commands: Commands,
    mut exit_events: EventReader<ExitEvent>,
    process: Query<Entity, With<ExitProcessMarker>>,
) {
    if let Some(ExitEvent) = exit_events.iter().last() {
        info!("ExitEvent received, spawning exit process");
        let process_entity = process.single();
        commands.entity(process_entity).insert(ExitProcess {
            timer: Timer::from_seconds(1.0, false),
        });
    }
}

fn exit_process(mut process: Query<(&mut ExitProcess, &mut ExitListener)>, time: Res<Time>) {
    if let Ok((mut process, mut listener)) = process.get_single_mut() {
        process.timer.tick(time.delta());

        if process.timer.just_finished() {
            info!("Fade out done, acknowledging exit");
            listener.acknowledge();
        }

        info!("Fading out: {:.2}%", 100. * process.timer.percent());
    }
}
