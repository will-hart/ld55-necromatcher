# Combat system

## Background

Based on [the brainstorming](./000_brainstorming.md), it would be great to have a combat system that:

- Allows combo effects,
- can cause friendly fire,
- results in counter-summons by the computer without any "AI" required

The overall system should be some sort of weird hybrid of checkers/chess and match3.

The objective is for the player to clear the board.

## Ideas

### Match 3 v1

- Maybe a match 3 style game where you get three in a row to remove them.
- The enemy "boss" needs to be destroyed to win
- every turn the enemy boss summons a shape
- if it gets three in a row something bad happens to you

### Match 3 v2 (from berru in bevy discord #jam channel)

- simple match 3
- when you match a red piece it disappears
- when reds disappear they spawn other pieces in the direction they're pointing
- probably only spawn into empty cells