#![allow(dead_code)]
use crate::{
    components_2d::*,
    ecs::{systems::ParallelRunnable, *},
};

pub fn build() -> impl ParallelRunnable {
    SystemBuilder::<()>::new("LocalToParentUpdateSystem2D")
        // Translation
        .with_query(
            <(Write<LocalToParent2>, Read<Translation2>)>::query()
                .filter(!component::<Rotation2>() & (maybe_changed::<Translation2>())),
        )
        // Rotation
        .with_query(
            <(Write<LocalToParent2>, Read<Rotation2>)>::query()
                .filter(!component::<Translation2>() & (maybe_changed::<Rotation2>())),
        )
        // Translation + Rotation
        .with_query(
            <(Write<LocalToParent2>, Read<Translation2>, Read<Rotation2>)>::query()
                .filter(maybe_changed::<Translation2>() | maybe_changed::<Rotation2>()),
        )
        .build(move |_commands, world, _, queries| {
            let (a, b, c) = queries;
            rayon::scope(|s| {
                s.spawn(|_| unsafe {
                    // Translation
                    a.for_each_unchecked(world, |(ltw, translation)| {
                        *ltw = LocalToParent2(translation.to_homogeneous());
                    });
                });
                s.spawn(|_| unsafe {
                    // Rotation
                    b.for_each_unchecked(world, |(ltw, rotation)| {
                        *ltw = LocalToParent2(rotation.to_homogeneous());
                    });
                });
                s.spawn(|_| unsafe {
                    // Translation + Rotation
                    c.for_each_unchecked(world, |(ltw, translation, rotation)| {
                        *ltw = LocalToParent2(
                            rotation
                                .to_homogeneous()
                                .append_translation(&translation),
                        );
                    });
                });
            });
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_parent_transformation() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut resources = Resources::default();
        let mut world = World::default();
        let prefab_world = World::default();
        let mut schedule = Schedule::builder().add_system(build()).build();

        let ltw = LocalToParent2::identity();
        let t = Translation2::new(1.0, 2.0);
        let r = Rotation2::identity();

        // Add every combination of transform types.
        let translation = world.push((ltw, t));
        let rotation = world.push((ltw, r));
        let translation_and_rotation = world.push((ltw, t, r));

        // Run the system
        schedule.execute(&mut world, &prefab_world, &mut resources);

        // Verify that each was transformed correctly.
        assert_eq!(
            world
                .entry(translation)
                .unwrap()
                .get_component::<LocalToParent2>()
                .unwrap()
                .0,
            t.to_homogeneous()
        );
        assert_eq!(
            world
                .entry(rotation)
                .unwrap()
                .get_component::<LocalToParent2>()
                .unwrap()
                .0,
            r.to_homogeneous()
        );
        assert_eq!(
            world
                .entry(translation_and_rotation)
                .unwrap()
                .get_component::<LocalToParent2>()
                .unwrap()
                .0,
            r.to_homogeneous().append_translation(&t),
        );
    }
}
