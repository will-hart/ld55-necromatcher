# LD 55 brainstorming

## Theme

"Summoning"

## Design constraints

The Jam starts half way through Saturday. With other life committments, I
have maybe 12 hours total to work on it. This requires keeping things very
simple.

- 2d vector line art
- very simple game play loop that can be implemented quickly
- able to extend "incrementally" by adding little pieces of content

## Ideas

- turn-based Team Fight Tactics, where you place combinations
  of troops / "runes" to beat opposing teams.
    - IDEA: placing runes side by side clears out areas
    - CON: requires some sort of AI which might be time consuming
  - EVOLUTION: a match three "chained-summoning" game,
    - game loop is you place items on a grid, the items react with nearby items
    - objective is to clear the grid of "enemy"
    - Placing items in specific combos results in a chain reaction of summoned items being placed nearby.
    - certain combinations of items produce bigger/smaller results
    - wrong combinations can harm yourself (summoning is dangerous right?)
    - PRO: doesn't require AI, just a "reducer" style state model
    - CON: may rely on lots of combinations that may be hard to "configure" or add incrementally
    - PRO: very simple art style
    - PRO: I've thought of it and it isn't a terrible idea and I have no time to linger.