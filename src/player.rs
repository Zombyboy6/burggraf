#![allow(dead_code)]
use avian3d::prelude::{Collider, RayCaster, RayHitData, RayHits, RigidBody, SpatialQueryFilter};
use bevy::{
    input::{ButtonState, keyboard::KeyboardInput, mouse::MouseMotion},
    prelude::*,
    render::view::{ColorGrading, ColorGradingGlobal, ColorGradingSection, Hdr},
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_inspector_egui::bevy_egui::PrimaryEguiContext;
use puppeteer::{
    puppet_rig::{PuppetRig, RelatedPuppet},
    puppeteer::{Puppeteer, PuppeteerInput},
};

use crate::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(
                Update,
                (mouse_lock, player_look, player_move, world_interaction)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, _asset_server: Res<AssetServer>) {
    let player_body = commands
        .spawn((
            Player,
            Puppeteer::default(),
            Collider::capsule(0.25, 1.80),
            RigidBody::Kinematic,
            Transform::from_xyz(0.0, 5.5, 0.0),
        ))
        .id();
    commands.spawn((
        PuppetRig {
            offset: Some(Vec3::new(0.0, 0.9, 0.0)),
            ..default()
        },
        RelatedPuppet::new(player_body),
        Camera3d::default(),
        // Raycaster for world interaction
        RayCaster::new(Vec3::ZERO, Dir3::NEG_Z)
            .with_ignore_self(true)
            .with_query_filter(SpatialQueryFilter::from_excluded_entities(vec![
                player_body,
            ]))
            .with_max_distance(3.0),
        Hdr,
        ColorGrading {
            global: ColorGradingGlobal {
                exposure: 0.0,
                temperature: -0.04,
                tint: 0.0,
                hue: 0.0,
                post_saturation: 1.05,
                midtones_range: 0.2..0.8,
            },
            shadows: ColorGradingSection {
                saturation: 1.0,
                contrast: 1.02,
                gamma: 1.0,
                gain: 1.0,
                lift: 0.0,
            },
            midtones: ColorGradingSection {
                saturation: 1.0,
                contrast: 1.00,
                gamma: 0.75,
                gain: 1.0,
                lift: 0.0,
            },
            highlights: ColorGradingSection {
                saturation: 1.0,
                contrast: 0.91,
                gamma: 0.75,
                gain: 1.0,
                lift: 0.0,
            },
        },
        PrimaryEguiContext,
    ));
}

fn mouse_lock(
    mut query: Query<&mut CursorOptions, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    let Ok(mut cursor_options) = query.single_mut() else {
        return;
    };

    if cursor_options.grab_mode != CursorGrabMode::Locked {
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    } else {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}
pub fn player_look(
    mut player_head_query: Query<&mut PuppetRig, Without<Player>>,
    mut mouse_motion_event: MessageReader<MouseMotion>,
    window: Single<&CursorOptions, With<PrimaryWindow>>,
) -> Result {
    let sensibility = 0.75;
    for mut head in player_head_query.iter_mut() {
        for mouse in mouse_motion_event.read() {
            if window.grab_mode == CursorGrabMode::None {
                continue;
            }
            head.pitch -= (0.1 * mouse.delta.y * sensibility).to_radians();
            head.yaw -= (0.1 * mouse.delta.x * sensibility).to_radians();

            head.pitch = head.pitch.clamp(-1.54, 1.54);
        }
    }
    Ok(())
}

pub fn player_move(
    player_head_query: Query<&PuppetRig>,
    mut player_query: Query<(&mut PuppeteerInput, &mut Puppeteer)>,
    mut keyboard_input: Local<ButtonInput<KeyCode>>,
    mut keyboard_input_events: MessageReader<KeyboardInput>,
) -> Result {
    keyboard_input.clear();
    for event in keyboard_input_events.read() {
        let key_code = event.key_code;
        match event.state {
            ButtonState::Pressed => keyboard_input.press(key_code),
            ButtonState::Released => keyboard_input.release(key_code),
        }
    }

    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec3::new(horizontal as f32, 0.0, vertical as f32).clamp_length_max(1.0);

    let head = player_head_query.single()?;
    let (mut input, _puppeteer) = player_query.single_mut()?;

    let local_z = Mat2::from_cols(
        [head.yaw.cos(), -head.yaw.sin()].into(),
        [head.yaw.sin(), head.yaw.cos()].into(),
    )
    .mul_vec2(Vec2::Y);
    let forward = -Vec3::new(local_z.x, 0., local_z.y);
    let right = Vec3::new(local_z.y, 0., -local_z.x);

    let mut move_vector = Vec3::ZERO;
    move_vector += forward * direction.z;
    move_vector += right * direction.x;
    move_vector = move_vector.normalize_or_zero();

    if keyboard_input.just_pressed(KeyCode::Space) {
        input.start_jump();
    }
    if keyboard_input.just_released(KeyCode::Space) {
        input.stop_jump();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        input.speed_multiplier = 2.0
    } else {
        input.speed_multiplier = 1.0
    }

    input.move_amount(move_vector);
    //println!("{:?}", move_vector);
    Ok(())
}

#[derive(EntityEvent)]
#[entity_event(propagate)]
#[entity_event(auto_propagate)]
pub struct PlayerInteraction {
    entity: Entity,
    hit: RayHitData,
}

#[derive(EntityEvent)]
#[entity_event(propagate)]
#[entity_event(auto_propagate)]
pub struct PlayerHit {
    entity: Entity,
    hit: RayHitData,
}

pub fn world_interaction(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    rays: Query<(&RayCaster, &RayHits), With<PuppetRig>>,
) {
    if !mouse_buttons.any_just_pressed(vec![MouseButton::Right, MouseButton::Left]) {
        return;
    }
    for (_ray_caster, ray_hits) in rays.iter() {
        let Some(first_hit) = ray_hits.first() else {
            continue;
        };

        if mouse_buttons.just_pressed(MouseButton::Right) {
            commands.trigger(PlayerInteraction {
                entity: first_hit.entity,
                hit: first_hit.clone(),
            });
        } else {
            commands.trigger(PlayerHit {
                entity: first_hit.entity,
                hit: first_hit.clone(),
            });
        };
    }
}
