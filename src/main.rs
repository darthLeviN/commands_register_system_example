use bevy::{
    ecs::system::{EntityCommands, SystemId},
    prelude::*,
    window::PrimaryWindow,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, startup_menu_system)
        .add_systems(Update, interactions)
        .run();
}

#[derive(Component)]
pub struct ButtonData {
    pub prev_interaction: Interaction,
    pub action: SystemId<(), ()>,
}

fn button_builder<'a, Marker: 'static>(
    commands: &'a mut Commands,
    action: impl IntoSystem<(), (), Marker> + 'static,
    asset_server: &AssetServer,
) -> EntityCommands<'a> {
    let new_sysid = commands.register_system(action);
    let mut ecmd = commands.spawn((
        ButtonData {
            prev_interaction: Interaction::None,
            action: new_sysid,
        },
        ButtonBundle {
            style: Style { ..default() },
            ..default()
        },
    ));

    ecmd.with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "BUTTON TEXT",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        });
    });

    return ecmd;
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn startup_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let button_entity = button_builder(
        &mut commands,
        |mut app_exit_event_writer: EventWriter<AppExit>| {
            println!("Hey we just pased a query to action and button was pressed. We can put any query here. it's flexible!");
        },
        &asset_server,
    )
    .id();

    commands
        .entity(main_menu_entity)
        .push_children(&[button_entity]);
}

fn interactions(
    mut commands: Commands,
    mut button_query: Query<(&mut ButtonData, &Interaction), Changed<Interaction>>,
) {
    for (mut button_data, interaction) in button_query.iter_mut() {
        if *interaction == Interaction::Hovered
            && button_data.prev_interaction == Interaction::Pressed
        {
            // Button was pressed.
            // *** This is the purpose of this PR. ***
            println!("Button pressed");
            commands.run_system(button_data.action);
        }
        button_data.prev_interaction = *interaction;
    }
}
