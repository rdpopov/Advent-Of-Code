# ideas
## Use iterative deepening search to go through next n states
- 1. Pick the number just enough to be killed if shield buff is not up. My case
     around 7
- 2. There is an explosion in states that should be considered, therefore prune
  them. I put them in map with keys made from current mana and the total mana
  spent and get the 2 most probable states to win. Most player hp and least boss
  hp. I reduce the states from thousands to hundreds. As a more aggressive
  stately just the total mana can be used

