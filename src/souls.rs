use crate::screens::Screen;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<QuickActionContext>()
        .add_systems(OnEnter(Screen::Gameplay), sync_quick_action_ui)
        .add_systems(Update, update_quick_action_ui.run_if(in_state(Screen::Gameplay)));
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ControlScheme {
    #[default]
    KeyboardMouse,
    Gamepad,
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct QuickActionContext {
    pub control_scheme: ControlScheme,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameAction {
    Move,
    Camera,
    Sprint,
    Dodge,
    LightAttack,
    HeavyAttack,
    LockOn,
    Interact,
    UseItem,
    SwitchTargetLeft,
    SwitchTargetRight,
}

impl GameAction {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Move => "Move",
            Self::Camera => "Camera",
            Self::Sprint => "Sprint",
            Self::Dodge => "Dodge",
            Self::LightAttack => "Attack",
            Self::HeavyAttack => "Heavy",
            Self::LockOn => "Lock On",
            Self::Interact => "Interact",
            Self::UseItem => "Use Item",
            Self::SwitchTargetLeft => "Target Left",
            Self::SwitchTargetRight => "Target Right",
        }
    }

    pub const fn binding_for(self, scheme: ControlScheme) -> &'static str {
        match (scheme, self) {
            (ControlScheme::KeyboardMouse, Self::Move) => "WASD",
            (ControlScheme::Gamepad, Self::Move) => "Left Stick",
            (ControlScheme::KeyboardMouse, Self::Camera) => "Mouse",
            (ControlScheme::Gamepad, Self::Camera) => "Right Stick",
            (ControlScheme::KeyboardMouse, Self::Sprint) => "Shift",
            (ControlScheme::Gamepad, Self::Sprint) => "L1 / R1",
            (ControlScheme::KeyboardMouse, Self::Dodge) => "Space",
            (ControlScheme::Gamepad, Self::Dodge) => "A",
            (ControlScheme::KeyboardMouse, Self::LightAttack) => "LMB",
            (ControlScheme::Gamepad, Self::LightAttack) => "RT",
            (ControlScheme::KeyboardMouse, Self::HeavyAttack) => "RMB",
            (ControlScheme::Gamepad, Self::HeavyAttack) => "LT",
            (ControlScheme::KeyboardMouse, Self::LockOn) => "Tab",
            (ControlScheme::Gamepad, Self::LockOn) => "R3",
            (ControlScheme::KeyboardMouse, Self::Interact) => "E",
            (ControlScheme::Gamepad, Self::Interact) => "X",
            (ControlScheme::KeyboardMouse, Self::UseItem) => "Q",
            (ControlScheme::Gamepad, Self::UseItem) => "Y",
            (ControlScheme::KeyboardMouse, Self::SwitchTargetLeft) => "Z",
            (ControlScheme::Gamepad, Self::SwitchTargetLeft) => "L1",
            (ControlScheme::KeyboardMouse, Self::SwitchTargetRight) => "C",
            (ControlScheme::Gamepad, Self::SwitchTargetRight) => "R1",
        }
    }
}

#[derive(Component)]
pub struct QuickActionHud;

fn sync_quick_action_ui(mut commands: Commands) {
    let hud = commands
        .spawn((
            QuickActionHud,
            Node {
                width: Val::Percent(35.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                right: Val::Px(24.0),
                top: Val::Px(24.0),
                padding: UiRect::all(Val::Px(12.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.03, 0.03, 0.06, 0.72)),
        ))
        .id();

    commands.entity(hud).with_children(|parent| {
        for action in [
            GameAction::Dodge,
            GameAction::LightAttack,
            GameAction::HeavyAttack,
            GameAction::LockOn,
            GameAction::Interact,
            GameAction::UseItem,
            GameAction::SwitchTargetLeft,
            GameAction::SwitchTargetRight,
            GameAction::Sprint,
        ] {
            let binding = action.binding_for(ControlScheme::KeyboardMouse);
            parent.spawn((
                Text::new(format!("[{}] {}", binding, action.label())),
                TextFont::from_font_size(16.0),
                TextColor::from(Color::srgb(0.95, 0.95, 0.95)),
            ));
        }
    });
}

fn update_quick_action_ui(
    context: Res<QuickActionContext>,
    mut hud_q: Query<(Entity, &Children), With<QuickActionHud>>,
    mut commands: Commands,
) {
    let Ok((hud_entity, children)) = hud_q.single_mut() else {
        return;
    };

    for child in children.iter() {
        commands.entity(child).despawn();
    }

    commands.entity(hud_entity).with_children(|parent| {
        for action in [
            GameAction::Dodge,
            GameAction::LightAttack,
            GameAction::HeavyAttack,
            GameAction::LockOn,
            GameAction::Interact,
            GameAction::UseItem,
            GameAction::SwitchTargetLeft,
            GameAction::SwitchTargetRight,
            GameAction::Sprint,
        ] {
            let binding = action.binding_for(context.control_scheme);
            parent.spawn((
                Text::new(format!("[{}] {}", binding, action.label())),
                TextFont::from_font_size(16.0),
                TextColor::from(Color::srgb(0.95, 0.95, 0.95)),
            ));
        }
    });
}
