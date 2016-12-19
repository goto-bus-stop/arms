import insert from table
import Map from require 'arms/Map'
import Player from require 'arms/Player'
import Trigger from require 'arms/Trigger'
import terrain, unit from require 'arms/constants'
import encode from require 'json'

class Arms
  new: =>
    @map = Map!
    @triggers = {}
    @players = {}

    -- Constants accessible as `Arms.something.constant`
    @terrain = terrain
    @unit = unit

    -- Bound aliases, usable as `Arms.method()` instead of `Arms:method()`
    @for_each_player = @\_for_each_player
    @trigger = @\_create_trigger

  _set_random_map_seed: (n) =>
    math.randomseed(n)
    @

  _set_number_of_players: (n) =>
    for i = 1, n do @players[i] = Player!

  _for_each_player: (callback) =>
    for player in *@players do callback(player)
    nil

  _create_trigger: (name) =>
    trig = Trigger name
    insert @triggers, trig
    return trig

  to_json: => {
    map: @map\to_json!
    triggers: [t\to_json! for t in *@triggers]
  }

  print: => print encode @to_json!

-- Exports
return Arms!
