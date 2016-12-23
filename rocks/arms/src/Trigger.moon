import insert from table
import DataObject from require 'arms.DataObject'

COND_NONE = 0
COND_BRING_OBJECT_TO_AREA = 1
COND_BNING_OBJECT_TO_OBJECT = 2
COND_OWN_OBJECTS = 3
COND_OWN_FEWER_OBJECTS = 4
COND_OBJECTS_IN_AREA = 5
COND_DESTROY_OBJECT = 6
COND_CAPTURE_OBJECT = 7
COND_ACCUMULATE_ATTRIBUTE = 8
COND_RESEARCH_TECHNOLOGY = 9
COND_TIMER = 10
COND_OBJECT_SELECTED = 11
COND_AI_SIGNAL = 12
COND_PLAYER_DEFEATED = 13
COND_OBJECT_HAS_TARGET = 14
COND_OBJECT_VISIBLE = 15
COND_OBJECT_NOT_VISIBLE = 16
COND_RESEARCHING_TECHNOLOGY = 17
COND_UNITS_GARRISONED = 18
COND_DIFFICULTY_LEVEL = 19

EFFECT_NONE = 0
EFFECT_CHANGE_DIPLOMACY = 1
EFFECT_RESEARCH_TECHNOLOGY = 2
EFFECT_SEND_CHAT = 3
EFFECT_PLAY_SOUND = 4
EFFECT_SEND_TRIBUTE = 5
EFFECT_UNLOCK_GATE = 6
EFFECT_LOCK_GATE = 7
EFFECT_ACTIVATE_TRIGGER = 8
EFFECT_DEACTIVATE_TRIGGER = 9
EFFECT_AI_SCRIPT_GOAL = 10
EFFECT_CREATE_OBJECT = 11
EFFECT_TASK_OBJECT = 12
EFFECT_DECLARE_VICTORY = 13
EFFECT_KILL_OBJECT = 14
EFFECT_REMOVE_OBJECT = 15
EFFECT_CHANGE_VIEW = 16
EFFECT_UNLOAD = 17
EFFECT_CHANGE_OWNERSHIP = 18
EFFECT_PATROL = 19
EFFECT_DISPLAY_INSTRUCTIONS = 20
EFFECT_CLEAR_INSTRUCTIONS = 21
EFFECT_FREEZE_UNIT = 22
EFFECT_USE_ADVANCED_BUTTONS = 23
EFFECT_DAMAGE_OBJECT = 24
EFFECT_PLACE_FOUNDATION = 25
EFFECT_CHANGE_OBJECT_NAME = 26
EFFECT_CHANGE_OBJECT_HP = 27
EFFECT_CHANGE_OBJECT_ATTACK = 28
EFFECT_STOP_UNIT = 29

-- Represents a location in a trigger condition or effect.
class Point
  new: (x = 0, y = 0) =>
    @x = x
    @y = y

-- Represents a rectangular area in a trigger condition or effect.
class Area
  new: (x1 = 0, y1 = 0, x2 = 0, y2 = 0) =>
    @x1 = x1
    @y1 = y1
    @x2 = x2
    @y2 = y2

-- Represents a trigger condition.
class Condition extends DataObject
  new: (type) => super
    type: type
    amount: nil -- Amount
    resource: nil -- Resource
    object_source: nil -- Object Source ID (Not same as effect)
    object_location: nil -- Object Location ID
    unit_constant: nil -- Unit Constant
    player: nil -- Player Source
    technology: nil -- Technology
    time: nil -- Time
    inverted: false -- Inverted for UP 1.4
    area: Area!
    unit_group: nil -- Unit Group
    unit_type: nil -- Unit Type
    ai_signal: nil -- AI Signal

-- Represents a Timer condition.
class ConditionTimer extends Condition
  new: => super COND_TIMER
  after: (seconds) => @set time: seconds

-- Builder for trigger conditions.
class Conditions
  new: =>
    @conditions = {}

  push: (obj) =>
    insert @conditions, obj
    return obj

  timer: (seconds) => @push ConditionTimer!\after seconds

  to_json: => [cond\to_json! for cond in *@conditions]

-- Represents a trigger effect.
class Effect extends DataObject
  new: (type) => super
    type: type -- Effect type
    goal: nil -- AI Goal
    amount: nil -- Amount
    resource: nil -- Resource
    diplomacy: nil -- Diplomacy
    object_location: nil -- Object Location ID
    unit_constant: nil -- Unit constant
    player: nil -- Player Source
    player_target: nil -- Player Target
    technology: nil -- Technology
    string_table: nil -- String Table
    unknown: nil -- Unknown
    time: nil -- Time
    trigger_index: nil -- Trigger Index
    location: Point!
    area: Area!
    unit_group: nil -- Unit Group
    unit_type: nil -- Unit Type
    panel: nil -- Panel
    text: nil -- Text
    sound: nil -- Sound
    object_ids: nil -- Objects Ids

-- Represents a "Kill Object" effect.
class EffectKillObjects extends Effect
  new: => super EFFECT_KILL_OBJECTS
  -- Set the player whose objects to kill.
  of_player: (player) => @set :player
  -- Set the type of objects to kill.
  of_type: (type) =>
    @set unit_type: switch type
      when 'building' then 2
      when 'civilian' then 3
      when 'military' then 4
      when 'other' then 1
      else nil
  -- Set the area in which to kill objects.
  in_area: (area) => @set :area

-- Represents a "Send Chat" effect.
class EffectChat extends Effect
  new: => super EFFECT_SEND_CHAT
  -- Set the target of the chat message.
  to: (player) => @set :player
  -- Set the contents of the chat message.
  text: (message) => @set text: message

-- Represents an "Activate Trigger" effect.
class EffectActivate extends Effect
  new: => super EFFECT_ACTIVATE_TRIGGER
  -- Set the trigger to activate.
  trigger: (trigger) => @set :trigger

-- Builder for trigger effects.
class Effects
  new: =>
    @effects = {}

  -- Add an effect.
  push: (obj) =>
    insert @effects, obj
    return obj

  -- Add a Send Chat effect.
  chat: (player, message) => @push EffectChat!\to(player)\text(message)
  -- Add a Kill Objects effect.
  kill_objects: => @push EffectKillObjects!
  -- Add an Activate Trigger effect.
  activate: (trigger) => @push EffectActivate!\trigger(trigger)

  -- Build a JSON-ready array of effects.
  to_json: => [effect\to_json! for effect in *@effects]

-- Represents a trigger.
class Trigger
  new: (name = "") =>
    @_name = name
    @_conditions = Conditions!
    @_effects = Effects!

  -- Configure trigger conditions using a callback.
  --
  --    trigger\conditions (condition) -> condition\timer(10)
  --
  conditions: (callback) =>
    callback @_conditions
    @

  -- Configure trigger effects using a callback.
  --
  --    trigger\effects (effect) -> effect\chat(1, 'Hello!')
  --
  effects: (callback) =>
    callback @_effects
    @

  to_json: => {
    name: @_name
    conditions: @_conditions\to_json!
    effects: @_effects\to_json!
  }

-- Exports
{ :Trigger }
