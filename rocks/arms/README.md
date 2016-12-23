# arms

A Lua library for writing JSON descriptions of Age of Empires 2 Scenario files.
Should one day be good at generating random maps as well.

```lua
Arms = require('arms')()
terrain = Arms.terrain

Arms.map:base_terrain(terrain.grass)
Arms.map:tile(0, 0):terrain(terrain.shallows)

Arms.trigger('Description')
  :conditions(function(condition) condition:timer(10) end)
  :effects(function(effect) effect:chat(1, 'Hello') end)

Arms:print()
```
