extern crate legion;
extern crate legion_transform;

use legion::*;
use legion_transform::prelude_3d::*;

#[allow(unused)]
fn tldr_sample() {
    // Create a normal Legion World
    let mut world = World::default();
    let mut resources = Resources::default();

    // Create a system bundle (vec of systems) for LegionTransform
    let transform_system_bundle = transform_system_bundle::build_3d();

    let parent_entity = world.push((
        // Always needed for an Entity that has any space transform
        LocalToWorld3::identity(),
        // The only mutable space transform a parent has is a translation.
        Translation3::new(100.0, 0.0, 0.0),
    ));

    world.extend(vec![
        (
            // Again, always need a `LocalToWorld` component for the Entity to have a custom
            // space transform.
            LocalToWorld3::identity(),
            // Here we define a Translation, Rotation and uniform Scale.
            Translation3::new(1.0, 2.0, 3.0),
            Rotation3::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
            Scale(2.0),
            // Add a Parent and LocalToParent component to attach a child to a parent.
            Parent(parent_entity),
            LocalToParent3::identity(),
        );
        4
    ]);
}

fn main() {
    // Create a normal Legion World
    let mut resources = Resources::default();
    let mut world = World::default();
    let prefab_world = World::default();

    // Create a system bundle (vec of systems) for LegionTransform
    let mut transform_system_bundle = transform_system_bundle::build_3d();

    // See `./types_of_transforms.rs` for an explanation of space-transform types.
    let parent_entity = world.push((
        LocalToWorld3::identity(),
        Translation3::new(100.0, 0.0, 0.0),
    ));

    let four_children: Vec<_> = world
        .extend(vec![
            (
                LocalToWorld3::identity(),
                Translation3::new(1.0, 2.0, 3.0),
                Rotation3::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
                Scale(2.0),
                // Add a Parent and LocalToParent component to attach a child to a parent.
                Parent(parent_entity),
                LocalToParent3::identity(),
            );
            4
        ])
        .iter()
        .cloned()
        .collect();

    // At this point the parent does NOT have a `Children` component attached to it. The `Children`
    // component is updated by the transform system bundle and thus can be out of date (or
    // non-existent for newly added members). By this logic, the `Parent` components should be
    // considered the always-correct 'source of truth' for any hierarchy.
    for system in transform_system_bundle.iter_mut() {
        system.prepare(&world);
        system.run(&mut world, &mut resources);
        system
            .command_buffer_mut(world.id())
            .unwrap()
            .flush(&mut world, &prefab_world);
    }

    // At this point all parents with children have a correct `Children` component.
    let parents_children = world
        .entry_ref(parent_entity)
        .unwrap()
        .get_component::<Children>()
        .unwrap()
        .0
        .clone();

    println!("Parent {:?}", parent_entity);
    for child in parents_children.iter() {
        println!(" -> Has child: {:?}", child);
    }

    // Each child will also have a `LocalToParent` component attached to it, which is a
    // space-transform from its local space to that of its parent.
    for child in four_children.iter() {
        println!("The child {:?}", child);
        println!(
            " -> Has a LocalToParent matrix: {}",
            world
                .entry_ref(*child)
                .unwrap()
                .get_component::<LocalToParent3>()
                .unwrap()
        );
        println!(
            " -> Has a LocalToWorld matrix: {}",
            world
                .entry_ref(*child)
                .unwrap()
                .get_component::<LocalToWorld3>()
                .unwrap()
        );
    }

    // Re-parent the second child to be a grandchild of the first.
    world
        .entry(four_children[1])
        .unwrap()
        .add_component(Parent(four_children[0]));

    // Re-running the system will cleanup and fix all `Children` components.
    for system in transform_system_bundle.iter_mut() {
        system.prepare(&world);
        system.run(&mut world, &mut resources);
        system
            .command_buffer_mut(world.id())
            .unwrap()
            .flush(&mut world, &prefab_world);
    }

    println!("After the second child was re-parented as a grandchild of the first child...");

    for child in world
        .entry_ref(parent_entity)
        .unwrap()
        .get_component::<Children>()
        .unwrap()
        .0
        .iter()
    {
        println!("Parent {:?} has child: {:?}", parent_entity, child);
    }

    for grandchild in world
        .entry_ref(four_children[0])
        .unwrap()
        .get_component::<Children>()
        .unwrap()
        .0
        .iter()
    {
        println!(
            "Child {:?} has grandchild: {:?}",
            four_children[0], grandchild
        );
    }

    println!("Grandchild: {:?}", four_children[1]);
    println!(
        " -> Has a LocalToWorld matrix: {}",
        world
            .entry_ref(four_children[1])
            .unwrap()
            .get_component::<LocalToWorld3>()
            .unwrap()
    );
}
