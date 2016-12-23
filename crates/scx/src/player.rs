use consts::Civilization;
use unit::Unit;

pub struct BaseResources {
    pub gold: u32,
    pub wood: u32,
    pub food: u32,
    pub stone: u32,
    pub ore: u32,
}

pub struct Player<'a> {
    pub name: &'a str,
    pub active: u32,
    pub human: u32,
    pub civilization: Civilization,
    pub resources: BaseResources,
    pub units: Vec<Unit>,
}

impl<'a> Player<'a> {
    pub fn empty<'b>() -> Player<'b> {
        Player {
            name: "",
            active: 0,
            human: 0,
            civilization: Civilization::None,
            resources: BaseResources {
                wood: 0,
                food: 0,
                gold: 0,
                stone: 0,
                ore: 0,
            },
            units: vec![],
        }
    }
}
