
# Initial Statement

Play the number game.


# Initial Refinement

  1. Ask the player to pick a number between 1 and 100.
  2. Pick 50 as an intial guess using the formula (100 - 1 + 1) / 2.
  3. Ask the player if his/her number is 50.
  4. If the player answers
    1. yes... stop the game.
    2. no... Ask the player if his/her number is lower or higher than 50.
      1. If the number is lower... Compute the next guess using (50 - 1 + 1) / 2
      2. If the number is higher... Compute the next guess using (100 - 50 + 1) / 2
      3. Return to step 3, using the new guess
