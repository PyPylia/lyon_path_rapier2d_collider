# lyon_path_rapier2d_collider
A simple library for converting a `lyon_tessellation` Path (or `bevy_prototype_lyon` Path/Geometry) into a `bevy_rapier2d` Collider.

Note: Trimeshes currently have some very weird collision physics, and may pass through objects. Convex hulls are preferred if applicable.

## Example
Check out `examples/hearts.rs` for a quick example.