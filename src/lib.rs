pub use legion as ecs;
pub use nalgebra as math;

pub mod components;
pub mod components_2d;
pub mod components_3d;
pub mod local_to_parent_system_3d;
pub mod local_to_parent_system_2d;
pub mod local_to_world_propagate_system_3d;
pub mod local_to_world_propagate_system_2d;
pub mod local_to_world_system_3d;
pub mod local_to_world_system_2d;
pub mod missing_previous_parent_system_3d;
pub mod missing_previous_parent_system_2d;
pub mod parent_update_system_3d;
pub mod parent_update_system_2d;
pub mod transform_system_bundle;

pub mod prelude_2d {
    pub use crate::components_2d::*;
    pub use crate::local_to_parent_system_2d;
    pub use crate::local_to_world_propagate_system_2d;
    pub use crate::local_to_world_system_2d;
    pub use crate::missing_previous_parent_system_2d;
    pub use crate::parent_update_system_2d;
    pub use crate::transform_system_bundle;
}

pub mod prelude_3d {
    pub use crate::components_3d::*;
    pub use crate::local_to_parent_system_3d;
    pub use crate::local_to_world_propagate_system_3d;
    pub use crate::local_to_world_system_3d;
    pub use crate::missing_previous_parent_system_3d;
    pub use crate::parent_update_system_3d;
    pub use crate::transform_system_bundle;
}