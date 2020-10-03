#![allow(dead_code)]
use crate::{
    components_2d::*,
    ecs::{
        systems::{CommandBuffer, ParallelRunnable},
        world::SubWorld,
        *,
    },
};

pub fn build() -> impl ParallelRunnable {
    SystemBuilder::<()>::new("LocalToWorldPropagateSystem2D")
        // Entities with a `Children` and `LocalToWorld` but NOT a `Parent` (ie those that are
        // roots of a hierarchy).
        .with_query(<(Read<Children>, Read<LocalToWorld2>)>::query().filter(!component::<Parent>()))
        .read_component::<Children>()
        .read_component::<LocalToParent2>()
        .build(move |commands, world, _resource, query| {
            for (children, local_to_world) in query.iter(world) {
                for child in children.0.iter() {
                    propagate_recursive(*local_to_world, world, *child, commands);
                }
            }
        })
}

fn propagate_recursive(
    parent_local_to_world: LocalToWorld2,
    world: &SubWorld,
    entity: Entity,
    commands: &mut CommandBuffer,
) {
    log::trace!("Updating LocalToWorld for {:?}", entity);
    let local_to_parent = {
        if let Some(local_to_parent) = world
            .entry_ref(entity)
            .ok()
            .and_then(|entry| entry.into_component::<LocalToParent2>().ok())
        {
            *local_to_parent
        } else {
            log::warn!(
                "Entity {:?} is a child in the hierarchy but does not have a LocalToParent",
                entity
            );
            return;
        }
    };

    let new_local_to_world = LocalToWorld2(parent_local_to_world.0 * local_to_parent.0);
    commands.add_component(entity, new_local_to_world);

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
        propagate_recursive(new_local_to_world, world, child, commands);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        local_to_parent_system_2d, local_to_world_propagate_system_2d, local_to_world_system_2d,
        missing_previous_parent_system_2d, parent_update_system_2d,
    };

    #[test]
    fn did_propagate() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut resources = Resources::default();
        let mut world = World::default();
        let prefab_world = World::default();

        let mut schedule = Schedule::builder()
            .add_system(missing_previous_parent_system_2d::build())
            .flush()
            .add_system(parent_update_system_2d::build())
            .flush()
            .add_system(local_to_parent_system_2d::build())
            .flush()
            .add_system(local_to_world_system_2d::build())
            .flush()
            .add_system(local_to_world_propagate_system_2d::build())
            .build();

        // Root entity
        let parent = world.push((Translation2::new(1.0, 0.0), LocalToWorld2::identity()));

        let children = world.extend(vec![
            (
                Translation2::new(0.0, 2.0),
                LocalToParent2::identity(),
                LocalToWorld2::identity(),
            ),
            (
                Translation2::new(0.0, 0.0),
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
