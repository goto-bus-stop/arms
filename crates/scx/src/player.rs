use consts::Civilization;
use unit::Unit;

pub struct BaseResources {
    pub gold: u32,
    pub wood: u32,
    pub food: u32,
    pub stone: u32,
    pub ore: u32,
}

pub struct Player {
    pub name: String,
    pub active: u32,
    pub human: u32,
    pub civilization: Civilization,
    pub resources: BaseResources,
    pub units: Vec<Unit>,
}

impl BaseResources {
    pub fn default() -> BaseResources {
        BaseResources {
            gold: 100,
            wood: 200,
            food: 200,
            stone: 200,
            ore: 0,
        }
    }
}

impl Player {
    pub fn empty() -> Player {
        Player {
            name: String::from(""),
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

    pub fn with_units(self, units: Vec<Unit>) -> Player {
        Player {
            name: self.name,
            active: self.active,
            human: self.human,
            civilization: self.civilization,
            resources: self.resources,
            units: units,
        }
    }
}
