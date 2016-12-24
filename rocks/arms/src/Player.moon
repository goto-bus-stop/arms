import DataObject from require 'arms.DataObject'

class Player extends DataObject
  new: (@owner, number) => super
    number: number
    name: "Player #{number}"
    civilization: math.random 1, 17

  place: (unit_type) =>
    unit = @owner\_create_unit unit_type
    unit\owner @data.number
    unit

class AIPlayer extends Player
  new: =>

-- Exports
{ :Player, :AIPlayer }
