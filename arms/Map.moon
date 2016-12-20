import terrain from require 'arms/constants'

class Map
  new: =>
    @size =
      x: 96
      y: 96
    @data =
      base_terrain: terrain.grass1
      base_elevation: 1
      tiles: {}

  base_terrain: (type) =>
    @data.base_terrain = type
  base_elevation: (level = 1) =>
    @data.base_elevation = level

  finalize: =>
    for y = 1, @size.y
      @data.tiles[y] or= {}
      for x = 1, @size.x
        @data.tiles[y][x] or= {
          t: @data.base_terrain
          e: @data.base_elevation
        }

  to_json: =>
    @finalize!

    return {
      size: { @size.x, @size.y }
      base_terrain: @data.base_terrain
      base_elevation: @data.base_elevation
      tiles: @data.tiles
    }

-- Exports
{ :Map }
