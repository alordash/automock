// TODO - think this should not be hidden in doc
#[doc(hidden)]
pub trait Mockable<'__ama> {
    fn id(&self) -> usize;
    
    type Setup;
    fn setup(&mut self) -> Self::Setup;

    type Received;
    fn received(&mut self) -> Self::Received;

    type StaticSetup;
    fn static_setup() -> Self::StaticSetup;

    type StaticReceived;
    fn static_received() -> Self::StaticReceived;
}
