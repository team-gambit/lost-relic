use bevy::prelude::*;

use crate::{
    collide::GameEvent,
    player::{Latency, Player},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ui)
            .add_system(update_latency_text);
    }
}

#[derive(Component)]
struct LatencyText;

#[derive(Component)]
struct LatencyImage;

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("VT323-Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(96.0), Val::Auto),
                        ..default()
                    },
                    image: asset_server.load("wifi_3.png").into(),
                    ..default()
                })
                .insert(LatencyImage);

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    text: Text::with_section(
                        "0ms",
                        TextStyle {
                            font,
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..default()
                        },
                    ),
                    ..default()
                })
                .insert(LatencyText);
        });
}

fn update_latency_text(
    asset_server: Res<AssetServer>,
    latency: Res<Latency>,
    mut text_query: Query<&mut Text, With<LatencyText>>,
    mut image_query: Query<&mut UiImage, With<LatencyImage>>,
    mut player_query: Query<&mut Player>,
    mut events: EventWriter<GameEvent>,
) {
    let mut text = text_query.single_mut();
    let mut image = image_query.single_mut();
    let player = match player_query.get_single_mut() {
        Ok(p) => p,
        Err(e) => {
            image.0 = asset_server.load("wifi_1.png");
            text.sections[0].value = "NaNms".to_string();
            return;
        }
    };

    match player.latency {
        0..=7 => image.0 = asset_server.load("wifi_3.png"),
        8..=24 => image.0 = asset_server.load("wifi_2.png"),
        25..=40 => image.0 = asset_server.load("wifi_1.png"),
        _ => events.send(GameEvent::Death),
    }

    text.sections[0].value = format!(
        "{:.0}ms",
        (latency.0.iter().sum::<i32>() as f32 / latency.0.len() as f32) * player.latency as f32
    );
}
