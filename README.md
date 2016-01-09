# Arms

Random Map Scripting for Age of Empires 2.

## What is this‽

It's mostly me trying out Rust!

In the future it'll hopefully be a Scenario file generator. You'll be able to
write Arms scripts in Lua or Moonscript, and the Arms app will output a Scenario
file for you.

It'll be aimed mostly at generating Random Map-style scenarios, like an
**A** dvanced **R** andom **M** ap **S** cripting language, with support for
generating triggers and very specific structures.

Something liiiike…

(VERY rough draft)

```lua
world.baseTerrain = Terrain.grass1

-- like "<PLAYER_SETUP> random_placement"
players:distribute_randomly()
-- but you could also do things like
-- player[1]:position(world.sizeX / 2, world.sizeY / 2)

-- generate elevation 😱
world.elevation
  -- keep it flat within 10 tiles of player starting positions
  -- this wouldn't be necessary on a nomad map, for example :)
  :avoid_players(10)
  -- Elevation.* are noise generators.
  :generate(Elevation.mountainy)

for player in players do
  local tc = player:place(Building.town_center):at(player.position)
  -- place three villagers at a random location, 3 tiles away from the town center
  -- using `player.starting_villagers` because it'll account for Chinese and
  -- Mayan bonuses.
  player
    :place(Unit.villager, player.starting_villagers)
    :near(tc, 3)
  -- place a scout 8 tiles away from the town center
  player:place(Unit.scout):near(tc, 8)
  -- place 6 sheep between 10 and 22 tiles away from the town center
  player:place(Unit.sheep, 6):near(tc, 10, 22)
end

-- generating triggers!
triggers.when(
  triggers.is_in_area(player, Unit.scout, get_an_area_description_somehow()),
  -- the `t` here will be some kind of magic trigger effect table
  function (t)
    t.kill_units_in_area()
  end
)
```

## References

This SCX file format struct on AoKHeaven:
http://aok.heavengames.com/cgi-bin/aokcgi/display.cgi?action=ct&f=4,40134,0,30

The [AgeScx](https://github.com/dderevjanik/AgeScx) library by [@dderevjanik](https://github.com/dderevjanik):
https://github.com/dderevjanik/AgeScx

## Licence

[MIT](./LICENSE)
