use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_lock_on_targets);
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CombatStats {
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub invulnerability: f32,
    pub poise: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Damage {
    pub amount: f32,
    pub poise_damage: f32,
    pub critical: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PoiseDamage(pub f32);

#[derive(Clone, Copy, Debug, Default)]
pub struct HitboxDefinition {
    pub radius: f32,
    pub half_height: f32,
    pub offset: Vec3,
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AttackPhase {
    pub startup: f32,
    pub active: f32,
    pub recovery: f32,
    pub stamina_cost: f32,
    pub damage: f32,
    pub poise_damage: f32,
}

#[derive(Clone, Debug, Default)]
pub struct AttackDefinition {
    pub name: &'static str,
    pub animation: &'static str,
    pub startup: f32,
    pub active: f32,
    pub recovery: f32,
    pub stamina_cost: f32,
    pub damage: f32,
    pub poise_damage: f32,
    pub hitbox: HitboxDefinition,
    pub combo_window: f32,
    pub movement_modifier: Vec3,
    pub rotation_behavior: &'static str,
    pub cancellation_rules: &'static str,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct LockOnTarget {
    pub entity: Entity,
    pub priority: f32,
    pub max_distance: f32,
    pub field_of_view: f32,
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct LockOnController {
    pub enabled: bool,
    pub target: Option<Entity>,
    pub max_distance: f32,
    pub field_of_view: f32,
    pub camera_follow: bool,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub amount: f32,
    pub poise_damage: f32,
}

fn update_lock_on_targets(mut targets: Query<(&mut LockOnTarget, &Transform)>) {
    for (mut target, transform) in &mut targets {
        target.max_distance = target.max_distance.max(0.1);
        target.field_of_view = target.field_of_view.clamp(0.0, 180.0);
        let _ = transform.translation;
    }
}
