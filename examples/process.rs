use bevy::log::info;
use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bevy_better_exit::{BetterExitPlugin, ExitEvent, ExitListener};
use std::time::Duration;

#[derive(Component)]
struct ExitProcess {
    timer: Option<Timer>,
}

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
    info!("Spawning ExitProcess");
    commands
        .spawn()
        .insert(ExitListener::new(Some("Process")))
        .insert(ExitProcess { timer: None });
    info!("Sending ExitEvent");
    exit.send(ExitEvent);
}

fn exit_listener(mut exit_events: EventReader<ExitEvent>, mut process: Query<&mut ExitProcess>) {
    if let Some(ExitEvent) = exit_events.iter().last() {
        info!("ExitEvent received, Enabling exit process timer");
        process.single_mut().timer = Some(Timer::from_seconds(1.0, false));
    }
}

fn exit_process(mut process: Query<(&mut ExitProcess, &mut ExitListener)>, time: Res<Time>) {
    let (mut process, mut listener) = process.single_mut();
    if let Some(ref mut timer) = &mut process.timer {
        timer.tick(time.delta());

        if timer.just_finished() {
            info!("Fade out done, acknowledging exit");
            listener.acknowledge();
        }

        info!("Fading out: {:.2}%", 100. * timer.percent());
    }
}
