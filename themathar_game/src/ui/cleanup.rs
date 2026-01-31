/// Cleanup utilities for UI

use bevy::prelude::*;

#[derive(Component)]
pub struct UIRoot;

pub(crate) fn cleanup_ui(
    mut commands: Commands,
    query: Query<Entity, With<UIRoot>>,
    children_query: Query<&Children>,
) {
    for entity in query.iter() {
        despawn_children_recursive(&mut commands, entity, &children_query);
    }
}

fn despawn_children_recursive(
    commands: &mut Commands,
    entity: Entity,
    children_query: &Query<&Children>,
) {
    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            despawn_children_recursive(commands, child, children_query);
        }
    }
    commands.entity(entity).despawn();
}
