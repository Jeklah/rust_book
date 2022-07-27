use std::{io::Read, fmt};

#[derive(Clone, Hash)]
pub enum Width {
    Int (i32),
    String (String),
}

#[derive(Clone, Hash)]
pub enum Height {
    Int (i32),
    String (String),
}

#[derive(Clone, Hash)]
pub struct Box {
    id: u8,
    height: Height,
    width: Width,
}
impl fmt::Debug for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Box")
            .field("id", &self.id)
            .field("height", &self.height)
            .field("width", &self.width)
            .finish()
    }
}
impl fmt::Display for Box {

}
            
impl Default for Box {
    fn default() -> Self {
        Self {
            id: 0,
            width: Width::String("three".to_string()),
            height: Height::Int(4),
        }
    }
}

fn main() {
    let firstbox = Box::default();
}
