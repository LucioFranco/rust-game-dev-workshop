mod animator;
mod input;
mod physics;
mod renderer;

// TODO: write a Phsyics system that takes `Motion` and `Position` and will apply
// physics too it!
// TODO: write AI system that takes `enemy` and `motion`.

pub use self::animator::Animator;
pub use self::input::Input;
pub use self::physics::Physics;
pub use self::renderer::Renderer;
