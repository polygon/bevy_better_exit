use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ElementState;
use bevy::prelude::*;

use bevy::log::{debug, error};

#[derive(Component)]
pub struct ExitListener {
    label: Option<&'static str>,
    ack: bool,
}

pub struct BetterExitPlugin(BetterExitData);

#[derive(Clone)]
struct BetterExitData {
    timeout: Option<f32>,
    exit_time: Option<f32>,
}

pub struct ExitEvent;

impl ExitListener {
    pub fn new(label: Option<&'static str>) -> Self {
        Self { label, ack: false }
    }

    pub fn acknowledge(&mut self) {
        self.ack = true;
    }
}

impl Default for BetterExitData {
    fn default() -> Self {
        BetterExitData {
            timeout: None,
            exit_time: None,
        }
    }
}

impl Plugin for BetterExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExitEvent>()
            .insert_resource(self.0.clone())
            .add_system(exit_event_handler)
            .add_system(exit_handler);
    }
}

impl BetterExitPlugin {
    pub fn new(timeout: Option<f32>) -> Self {
        BetterExitPlugin(BetterExitData {
            timeout,
            ..Default::default()
        })
    }
}

fn exit_event_handler(
    mut ev: EventReader<ExitEvent>,
    time: Res<Time>,
    mut data: ResMut<BetterExitData>,
) {
    if let Some(ExitEvent) = ev.iter().last() {
        if data.exit_time.is_none() {
            data.exit_time = Some(time.seconds_since_startup() as f32);
            debug!(
                "ExitEvent received at {:.2}s, waiting for listeners",
                data.exit_time.unwrap()
            );
        }
    }
}

fn exit_handler(
    listeners: Query<&ExitListener>,
    data: Res<BetterExitData>,
    time: Res<Time>,
    mut exit: EventWriter<AppExit>,
) {
    if let Some(exit_time) = data.exit_time {
        let all_done = !listeners.iter().any(|l| l.ack == false);
        if let (false, Some(timeout)) = (all_done, data.timeout) {
            if time.seconds_since_startup() as f32 - exit_time > timeout {
                for l in listeners.iter().filter(|l| l.ack == false) {
                    error!(
                        "Exit listener did not confirm in time: {}",
                        l.label.unwrap_or("(anonymous)")
                    );
                }
                error!("Exiting after timeout");
                exit.send(AppExit);
            }
        }

        if all_done {
            debug!("All listeners finished, exiting application");
            exit.send(AppExit);
        }
    }
}

pub fn exit_on_esc_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut exit_events: EventWriter<ExitEvent>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed && key_code == KeyCode::Escape {
                exit_events.send(ExitEvent);
            }
        }
    }
}
