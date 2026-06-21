//! Smoke test verifying TemplateGame components reach the editor's component picker.

#![cfg(feature = "editor")]

use std::collections::HashSet;

use bevy::prelude::*;
use jackdaw::inspector::component_picker::{PickerDenylist, enumerate_pickable_components};

#[test]
fn user_components_reach_picker() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(template_game::TemplateGamePlugin);

    let registry = app
        .world()
        .resource::<bevy::ecs::reflect::AppTypeRegistry>()
        .read();
    let denied_components = HashSet::new();
    let picker_denylist = PickerDenylist::default();
    let pickable_components =
        enumerate_pickable_components(&registry, &denied_components, &picker_denylist);
    let pickable_component_names: Vec<&str> = pickable_components
        .iter()
        .map(|pickable_component| pickable_component.short_name.as_str())
        .collect();

    assert!(
        pickable_component_names.contains(&"SpinningCube"),
        "SpinningCube must appear in the editor's component picker. Available: {pickable_component_names:?}",
    );
}
