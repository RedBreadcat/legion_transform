#![allow(dead_code)]
use crate::{
    components_2d::*,
    ecs::{systems::ParallelRunnable, *},
};

// TODO: this doesn't really do anything

pub fn build() -> impl ParallelRunnable {
    SystemBuilder::<()>::new("LocalToParentUpdateSystem2D")
        // Translation + Rotation
        .with_query(
            <(
                Write<LocalToParent2>,
                Read<LocalTranslation2>,
                Read<LocalRotation2>,
            )>::query()
            .filter(maybe_changed::<LocalTranslation2>() | maybe_changed::<LocalRotation2>()),
        )
        .build(move |_commands, world, _, queries| {
            let a = queries;
            rayon::scope(|s| {
                s.spawn(|_| unsafe {
                    a.for_each_unchecked(world, |(ltp, translation, rotation)| {
                        *ltp = LocalToParent2(
                            rotation
                                .0
                                .to_homogeneous()
                                .append_translation(&(translation.0).0),
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
        let t = LocalTranslation2(Translation2::new(1.0, 2.0));
        let r = LocalRotation2::from(0.0);

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
            t.0.to_homogeneous()
        );
        assert_eq!(
            world
                .entry(rotation)
                .unwrap()
                .get_component::<LocalToParent2>()
                .unwrap()
                .0,
            r.0.to_homogeneous()
        );
        assert_eq!(
            world
                .entry(translation_and_rotation)
                .unwrap()
                .get_component::<LocalToParent2>()
                .unwrap()
                .0,
            r.0.to_homogeneous().append_translation(&(t.0).0),
        );
    }
}
