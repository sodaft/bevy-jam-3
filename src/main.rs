use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(get_default_plugins())
        .add_startup_system(setup)
        .run();
}

fn get_default_plugins() -> impl PluginGroup {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    })
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
