import terrain from require 'arms.constants'

class Tile
  new: (@terrain_type, @elevation) =>
  to_json: => { t: @terrain_type, e: @elevation }

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

  tile: (x, y) =>
    @data.tiles[y] or= {}
    @data.tiles[y][x] or= Tile @data.base_terrain, @data.base_elevation
    @data.tiles[y][x]

  finalize: =>
    for y = 1, @size.y
      @data.tiles[y] or= {}
      for x = 1, @size.x
        @data.tiles[y][x] or= Tile @data.base_terrain, @data.base_elevation

  to_json: =>
    @finalize!

    return {
      size: { @size.x, @size.y }
      base_terrain: @data.base_terrain
      base_elevation: @data.base_elevation
      tiles: [ [ tile\to_json! for tile in *row ] for row in *@data.tiles ]
    }

-- Exports
{ :Map }
