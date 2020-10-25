#![allow(dead_code)]
use crate::{
    components_2d::*,
    ecs::{systems::CommandBuffer, world::SubWorld, *},
};

// TODO: what's the point of localtoworld? All I need is the static localtoparent?
#[system(for_each)]
#[filter(!component::<Parent>())]
#[read_component(LocalToParent2)]
#[write_component(Translation2)]
#[write_component(Rotation2)]
#[write_component(LocalTranslation2)]
#[write_component(LocalRotation2)]
pub fn local_to_world_propagate(
    world: &mut SubWorld,
    commands: &mut CommandBuffer,
    children: &Children,
    translation: &Translation2,
    rotation: &Rotation2,
) {
    let local_to_world =
        LocalToWorld2(rotation.to_homogeneous().append_translation(&translation.0));
    for child in children.0.iter() {
        propagate_recursive(&local_to_world, world, *child, commands);
    }
}

fn propagate_recursive(
    parent_local_to_world: &LocalToWorld2,
    world: &mut SubWorld,
    entity: Entity,
    commands: &mut CommandBuffer,
) {
    let local_to_parent = {
        let entry = world.entry_ref(entity).unwrap();
        *entry.get_component::<LocalToParent2>().unwrap()
    };

    let new_local_to_world = LocalToWorld2(parent_local_to_world.0 * local_to_parent.0);

    {
        let mut entry = world.entry_mut(entity).unwrap();
        let local_translation = *entry.get_component::<LocalTranslation2>().unwrap();
        *entry.get_component_mut::<Translation2>().unwrap() =
            local_translation.0.transform_to_copy(&new_local_to_world.0);
        let local_rotation = *entry.get_component::<LocalRotation2>().unwrap();
        *entry.get_component_mut::<Rotation2>().unwrap() =
            local_rotation.0.transform_to_copy(&new_local_to_world.0);
    }

    // Collect children
    let children = if let Ok(entry) = world.entry_ref(entity) {
        entry
            .get_component::<Children>()
            .map(|e| e.0.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    } else {
        Vec::default()
    };

    for child in children {
        propagate_recursive(&new_local_to_world, world, child, commands);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        local_to_parent_system_2d,
        local_to_world_propagate_system_2d, /*local_to_world_system_2d,
                                           missing_previous_parent_system_2d, parent_update_system_2d,*/
    };

    #[test]
    fn did_propagate() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut resources = Resources::default();
        let mut world = World::default();
        let prefab_world = World::default();

        let mut schedule = Schedule::builder()
            //.add_system(missing_previous_parent_system_2d::build())
            //.flush()
            //.add_system(parent_update_system_2d::build())
            //.flush()
            .add_system(local_to_parent_system_2d::build())
            .flush()
            //.add_system(local_to_world_system_2d::build())
            //.flush()
            .add_system(local_to_world_propagate_system_2d::build())
            .build();

        // Root entity
        let parent = world.push((
            LocalTranslation2(Translation2::new(1.0, 0.0)),
            LocalToWorld2::identity(),
        ));

        let children = world.extend(vec![
            (
                LocalTranslation2(Translation2::new(0.0, 2.0)),
                LocalToParent2::identity(),
                LocalToWorld2::identity(),
            ),
            (
                LocalTranslation2(Translation2::new(0.0, 0.0)),
                LocalToParent2::identity(),
                LocalToWorld2::identity(),
            ),
        ]);
        let (e1, e2) = (children[0], children[1]);

        // Parent `e1` and `e2` to `parent`.
        world.entry(e1).unwrap().add_component(Parent(parent));
        world.entry(e2).unwrap().add_component(Parent(parent));

        // Run systems
        schedule.execute(&mut world, &prefab_world, &mut resources);

        assert_eq!(
            world
                .entry(e1)
                .unwrap()
                .get_component::<LocalToWorld2>()
                .unwrap()
                .0,
            Translation2::new(1.0, 0.0).to_homogeneous()
                * Translation2::new(0.0, 2.0).to_homogeneous()
        );

        assert_eq!(
            world
                .entry(e2)
                .unwrap()
                .get_component::<LocalToWorld2>()
                .unwrap()
                .0,
            Translation2::new(1.0, 0.0).to_homogeneous()
                * Translation2::new(0.0, 0.0).to_homogeneous()
        );
    }
}
