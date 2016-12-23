use selection::{Coordinate, Rectangle};

pub struct TriggerCondition {
    pub condition: i32,
    pub check: i32,
    pub amount: i32,
    pub resource: i32,
    pub unit_object: i32,
    pub unit_location: i32,
    pub unit_type: i32,
    pub player: i32,
    pub technology: i32,
    pub timer: i32,
    pub area: Rectangle,
    pub unit_group: i32,
    pub ai_signal: i32,
}

pub struct TriggerEffect<'a> {
    pub effect: i32,
    pub check: i32,
    pub ai_goal: i32,
    pub amount: i32,
    pub resource: i32,
    pub diplomacy: i32,
    pub unit_location: i32,
    pub unit_type: i32,
    pub player_source: i32,
    pub player_target: i32,
    pub technology: i32,
    pub text_id: i32,
    pub display_time: i32,
    pub trigger_index: i32,
    pub location: Coordinate,
    pub area: Rectangle,
    pub unit_group: i32,
    pub object_type: i32,
    pub instruction_panel: i32,
    pub text: &'a str,
    pub sound_filename: &'a str,
    pub unit_ids: Vec<i32>,
}

pub struct Trigger<'a> {
    pub enabled: bool,
    pub is_looping: bool,
    pub is_objective: bool,
    pub name: &'a str,
    pub description: &'a str,
    pub conditions: Vec<TriggerCondition>,
    pub effects: Vec<TriggerEffect<'a>>,
}
