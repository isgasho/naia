use std::{
    any::TypeId,
    fmt::{Debug, Formatter, Result},
};

use super::event_type::EventType;

/// Handles the creation of new Events
pub trait EventBuilder<T: EventType> {
    /// Gets the TypeId of the Event it is able to build
    fn get_type_id(&self) -> TypeId;
    /// Creates a new Event
    fn build(&self, in_bytes: &[u8]) -> T;
}

impl<T: EventType> Debug for Box<dyn EventBuilder<T>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("Boxed EventBuilder")
    }
}
