use super::*;
use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

pub fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>()
        .add_observer(on_modal_add);
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct ZoomView;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Sprint;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Dodge;

#[derive(InputAction)]
#[action_output(bool)]
pub struct LightAttack;

#[derive(InputAction)]
#[action_output(bool)]
pub struct HeavyAttack;

#[derive(InputAction)]
#[action_output(bool)]
pub struct LockOn;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Interact;

#[derive(InputAction)]
#[action_output(bool)]
pub struct UseItem;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SwitchTargetLeft;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SwitchTargetRight;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Dash;

#[derive(Component, Default)]
#[component(on_add = PlayerInput::on_add)]
pub(crate) struct PlayerInput;

impl PlayerInput {
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        world.commands().entity(ctx.entity).insert((
            actions!(PlayerInput[
                (
                    Action::<Movement>::new(),
                    DeadZone::default(),
                    Scale::splat(0.3),
                    Bindings::spawn((
                        Cardinal::wasd_keys(),
                        Cardinal::arrows(),
                        Axial::left_stick(),
                    )),
                ),
                (
                    Action::<Crouch>::new(),
                    ActionSettings {
                        require_reset: true,
                        ..Default::default()
                    },
                    bindings![KeyCode::ControlLeft, GamepadButton::East],
                ),
                (
                    Action::<Jump>::new(),
                    bindings![KeyCode::Space, GamepadButton::South],
                ),
                (
                    Action::<Dodge>::new(),
                    ActionSettings {
                        require_reset: true,
                        ..Default::default()
                    },
                    bindings![KeyCode::ShiftLeft, GamepadButton::South],
                ),
                (
                    Action::<Sprint>::new(),
                    bindings![KeyCode::ShiftLeft, GamepadButton::LeftThumb],
                ),
                (
                    Action::<LightAttack>::new(),
                    bindings![MouseButton::Left, GamepadButton::RightTrigger],
                ),
                (
                    Action::<HeavyAttack>::new(),
                    bindings![MouseButton::Right, GamepadButton::LeftTrigger],
                ),
                (
                    Action::<LockOn>::new(),
                    ActionSettings {
                        require_reset: true,
                        ..Default::default()
                    },
                    bindings![KeyCode::Tab, GamepadButton::RightThumb],
                ),
                (
                    Action::<Interact>::new(),
                    bindings![KeyCode::KeyE, GamepadButton::West],
                ),
                (
                    Action::<UseItem>::new(),
                    bindings![KeyCode::KeyQ, GamepadButton::North],
                ),
                (
                    Action::<SwitchTargetLeft>::new(),
                    bindings![KeyCode::KeyZ, GamepadButton::LeftTrigger],
                ),
                (
                    Action::<SwitchTargetRight>::new(),
                    bindings![KeyCode::KeyC, GamepadButton::RightTrigger],
                ),
                (
                    Action::<RotateCamera>::new(),
                    Bindings::spawn((
                        // tweak mouse and right stick sensitivity in Scale::splat values
                        Spawn((Binding::mouse_motion(), Scale::splat(0.07))),
                        Axial::right_stick().with((Scale::splat(4.0), DeadZone::default())),
                    )),
                ),
                (
                    Action::<Escape>::new(),
                    ActionSettings {
                        require_reset: true,
                        ..Default::default()
                    },
                    bindings![KeyCode::Escape, GamepadButton::Select],
                )
            ]),
            ModalInput,
            ContextActivity::<ModalInput>::INACTIVE,
            ContextActivity::<PlayerInput>::ACTIVE,
        ));
    }
}

fn on_modal_add(_: On<Add, Modal>, mut commands: Commands, players_q: Query<Entity, With<Player>>) {
    // TODO: only do that for the player that called the modal
    for e in players_q.iter() {
        commands.entity(e).insert_if_new(modal_ctx_active());
    }
}

pub fn modal_ctx_active() -> impl Bundle {
    (
        ContextActivity::<ModalInput>::ACTIVE,
        ContextActivity::<PlayerInput>::INACTIVE,
    )
}
pub fn player_ctx_active() -> impl Bundle {
    (
        ContextActivity::<ModalInput>::INACTIVE,
        ContextActivity::<PlayerInput>::ACTIVE,
    )
}
