use dygma_focus::Focus;
use std::sync::Mutex;

#[derive(Default)]
pub(crate) struct Storage {
    pub focus: Mutex<Option<Focus>>,
}
