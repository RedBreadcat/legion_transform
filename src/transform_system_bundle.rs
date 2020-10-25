use crate::{
    ecs::systems::ParallelRunnable, local_to_parent_system_2d, local_to_world_propagate_system_2d,
    parent_update_system_2d,
};

pub fn build_2d() -> Vec<Box<dyn ParallelRunnable>> {
    let mut all_systems = Vec::<Box<dyn ParallelRunnable>>::with_capacity(3);
    all_systems.push(Box::new(local_to_parent_system_2d::build()));
    all_systems.push(Box::new(parent_update_system_2d::parent_update_system()));
    all_systems.push(Box::new(
        local_to_world_propagate_system_2d::local_to_world_propagate_system(),
    ));

    all_systems
}
