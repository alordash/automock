/// Interface of every mock.
pub trait Mockable<'__ama> {
    /// Returns id of this mock. Each mock object has unique id, regardless of mock type.
    fn id(&self) -> usize;

    /// Struct used to set up mock.
    type Setup;
    /// Begins mock set up.
    fn setup(&mut self) -> Self::Setup;

    /// Struct used to set up mock's static functions.
    type StaticSetup;
    /// Begins mock's static functions set up.
    fn static_setup() -> Self::StaticSetup;
}
