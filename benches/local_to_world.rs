#![feature(test)]

extern crate test;

use legion::*;
use legion_transform::prelude_3d::*;
use test::Bencher;

#[bench]
fn local_to_world_update_without_change(b: &mut Bencher) {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut resources = Resources::default();
    let mut world = World::default();
    let prefab_world = World::default();
    let mut schedule = Schedule::builder()
        .add_system(local_to_world_system_3d::build())
        .build();

    let ltw = LocalToWorld3::identity();
    let t = Translation3::new(1.0, 2.0, 3.0);
    let r = Rotation3::from_euler_angles(1.0, 2.0, 3.0);
    let s = Scale(2.0);
    let nus = NonUniformScale::new(1.0, 2.0, 3.0);

    // Add N of every combination of transform types.
    let n = 1000;
    world.extend(vec![(ltw, t); n]);
    world.extend(vec![(ltw, r); n]);
    world.extend(vec![(ltw, s); n]);
    world.extend(vec![(ltw, nus); n]);
    world.extend(vec![(ltw, t, r); n]);
    world.extend(vec![(ltw, t, s); n]);
    world.extend(vec![(ltw, t, nus); n]);
    world.extend(vec![(ltw, r, s); n]);
    world.extend(vec![(ltw, r, nus); n]);
    world.extend(vec![(ltw, t, r, s); n]);
    world.extend(vec![(ltw, t, r, nus); n]);

    // Run the system once outside the test (which should compute everything and it shouldn't be
    // touched again).
    schedule.execute(&mut world, &prefab_world, &mut resources);

    // Then time the already-computed updates.
    b.iter(|| {
        schedule.execute(&mut world, &prefab_world, &mut resources);
    });
}
