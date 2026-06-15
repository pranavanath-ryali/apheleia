/// Defines the stage at which a system runs and what capabilities it can access
///
/// Each tick follows a pipeline:
/// ```text
///     Event -> Update -> Render
/// ```
///
/// # Stages
/// 1. [`Event`]: Can access raw or external events provided by _crossterm_
/// 2. [`Update`]: No extra capabilities. This is the stage where most systems are expected to run in.
/// 3. [`Render`]: Can access the [`Buffer`] for rendering capability.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum SystemRunStage {
    Event,
    Update,
    Render,
}
