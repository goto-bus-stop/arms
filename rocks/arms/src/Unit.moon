import DataObject from require 'arms.DataObject'

unit_id = 0
next_id = ->
  unit_id = unit_id + 1
  unit_id

class Unit extends DataObject
  new: (type) => super
    id: next_id!
    type: type
    x: 0
    y: 0
    angle: 0
    frame: 0
    garrison_id: 0
    owner: nil

  -- Set the unit's owner. `nil` for gaia.
  owner: (owner) => @set owner: owner
  -- Set the unit's location.
  at: (x, y) => @set x: x, y: y
  -- Set the unit's rotation.
  rotation: (angle) => @set angle: angle

-- Exports
{ :Unit }