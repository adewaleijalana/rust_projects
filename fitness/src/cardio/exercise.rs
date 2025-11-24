use crate::cardio::cardio_tool::CardioTool;

#[derive(Debug)]
pub struct Exercise {
     day: String,  
     tool: CardioTool,
     minute: u32
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minute: u32) -> Self {
        Self { day, tool, minute }
    }
} 