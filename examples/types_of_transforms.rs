extern crate legion;
extern crate legion_transform;

use legion::*;
use legion_transform::prelude_3d::*;

fn main() {
    // Create a normal Legion World
    let mut world = World::default();
    let prefab_world = World::default();
    let mut resources = Resources::default();

    // Create a system bundle (vec of systems) for LegionTransform
    let mut transform_system_bundle = transform_system_bundle::build_3d();

    // A user-defined space transform is split into 4 different components: [`Translation`,
    // `Rotation`, `Scale`, `NonUniformScale`]. Any combination of these components can be added to
    // an entity to transform it's space (exception: `Scale` and `NonUniformScale` are mutually
    // exclusive).

    // Note that all entities need an explicitly added `LocalToWorld` component to be considered for
    // processing during transform system passes.

    // Add an entity with just a Translation
    // See: https://www.nalgebra.org/rustdoc/nalgebra/geometry/struct.Translation.html
    // API on Translation, as a LegionTransform `Translation` is just a nalgebra `Translation3`.
    world.push((LocalToWorld3::identity(), Translation3::new(1.0, 2.0, 3.0)));

    // Add an entity with just a Rotation.
    // See: https://www.nalgebra.org/rustdoc/nalgebra/geometry/type.UnitQuaternion.html for the full
    // API on Rotation, as a LegionTransform `Rotation` is just a nalgebra `UnityQuaternion`.
    world.push((
        LocalToWorld3::identity(),
        Rotation3::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
    ));

    // Add an entity with just a uniform Scale (the default and strongly-preferred scale component).
    // This is simply a `f32` wrapper.
    world.push((LocalToWorld3::identity(), Scale(2.0)));

    // Add an entity with just a NonUniformScale (This should be avoided unless you **really** need
    // non-uniform scaling as it breaks things like physics colliders.
    // See: https://docs.rs/nalgebra/0.10.1/nalgebra/struct.Vector3.html for the full API on
    // NonUniformScale, as a LegionTransform `NonUniformScale` is simply a nalgebra `Vector3`,
    // although note that it is wrapped in a tuple-struct.
    world.push((
        LocalToWorld3::identity(),
        NonUniformScale::new(1.0, 2.0, 1.0),
    ));

    // Add an entity with a combination of Translation and Rotation
    world.push((
        LocalToWorld3::identity(),
        Translation3::new(1.0, 2.0, 3.0),
        Rotation3::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
    ));

    // Add an entity with a combination of Translation and Rotation and uniform Scale.
    world.push((
        LocalToWorld3::identity(),
        Translation3::new(1.0, 2.0, 3.0),
        Rotation3::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
        Scale(2.0),
    ));

    // Run the system bundle (this API will likely change).
    for system in transform_system_bundle.iter_mut() {
        system.prepare(&world);
        system.run(&mut world, &mut resources);
        system
            .command_buffer_mut(world.id())
            .unwrap()
            .flush(&mut world, &prefab_world);
    }

    // At this point all `LocalToWorld` components have correct values in them. Running the system
    // again will result in a short-circuit as only changed components are considered for update.
    let mut query = <(Entity, Read<LocalToWorld3>)>::query();
    for (entity, local_to_world) in query.iter(&mut world) {
        println!(
            "Entity {:?} and a LocalToWorld matrix: {}",
            entity, *local_to_world
        );
    }
}
