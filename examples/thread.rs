use bevy::{
    app::ScheduleRunnerSettings, log::info, log::LogPlugin, prelude::*, tasks::AsyncComputeTaskPool,
};
use bevy_better_exit::{BetterExitPlugin, ExitEvent, ExitListener};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Component)]
struct ThreadExit(Arc<AtomicBool>);

struct ExitTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .insert_resource(ExitTimer(Timer::from_seconds(5.0, false)))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(BetterExitPlugin::new(None))
        .add_startup_system(setup)
        .add_system(exit_after_5s)
        .add_system(exit_listener)
        .run();
}

fn setup(mut commands: Commands, pool: ResMut<AsyncComputeTaskPool>) {
    info!("Starting worker thread");
    let thread_exit = Arc::new(AtomicBool::new(false));
    pool.spawn(worker(Arc::clone(&thread_exit))).detach();
    commands
        .spawn()
        .insert(ExitListener::new(Some("Worker")))
        .insert(ThreadExit(thread_exit));
}

async fn worker(thread_exit: Arc<AtomicBool>) {
    info!("In: Worker thread");
    while !thread_exit.load(Ordering::SeqCst) {}
    info!("Worker thread terminating");
}

fn exit_after_5s(mut timer: ResMut<ExitTimer>, time: Res<Time>, mut exit: EventWriter<ExitEvent>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        info!("Sending ExitEvent");
        exit.send(ExitEvent);
    }
}

fn exit_listener(
    mut thread_exit: Query<(&mut ThreadExit, &mut ExitListener)>,
    mut ev: EventReader<ExitEvent>,
) {
    if let Some(ExitEvent) = ev.iter().last() {
        let (thread_exit, mut listener) = thread_exit.single_mut();
        thread_exit.0.store(true, Ordering::SeqCst);
        listener.acknowledge();
    }
}
