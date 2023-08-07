# Some notes on the solution
- the state is represented by a single 64 bit integer,
  - the separate floors are accessed by a union.
  - this should be also VERY fast to copy, to produce new states or pass around.
  - this should be also VERY fast to to checks if the state is valid, as it is a
    simple bitwise operation.
  - Format: 1 bit for where the elevator is, 7 for each floor for chips and
    generators. The chips have one bit in abundance. This might be useful.

- When new states are generated only the smallest are kept as they have items at
  the highest, therefore closer to solution. (bits in the least significant
  part.) But all generated states are counted as visited. We never have to go
  back to those when we are deeper. Only a few are kept - 1000, can go as low as
  400, and it is fast, but might be unreliable depending on input.

- States are generated this way. 
    - we have a counter from 1. If all those pass:
        - if we can move it up or down and it has 2 or 1 raised bits (moving 1 or 2 things)
        - bitwise AND with generators or chips passes
        - if it has only 1 raised it is also moved.
        - if it has only one raised and both checks for chips and generators
          pass, then both are moved
## Improvements that can be made

- The moves needed to push n elements up a floor, is defined by this formula: 2*(n-1)-1.
  Since this is already solved, if we get all elements to the highest floor
  there are elements(in the example case that would be the third floor). We can
  stop there, and return the sum of the formula and the current moves made.

- The bit that is wasted with the chips, can be used to mark the desired floor.




F4 .  .  .  .  .  
F3 .  .  .  LG .  
F2 .  HG .  .  .  
F1 E  .  HM .  LM 

Then, to get everything up to the assembling machine on the fourth floor, the following steps could be taken:

    Bring the Hydrogen-compatible Microchip to the second floor, which is safe because it can get power from the Hydrogen Generator:

    F4 .  .  .  .  .  
    F3 .  .  .  LG .  
    F2 E  HG HM .  .  
    F1 .  .  .  .  LM 

    Bring both Hydrogen-related items to the third floor, which is safe because the Hydrogen-compatible microchip is getting power from its generator:

    F4 .  .  .  .  .  
    F3 E  HG HM LG .  
    F2 .  .  .  .  .  
    F1 .  .  .  .  LM 

    Leave the Hydrogen Generator on floor three, but bring the Hydrogen-compatible Microchip back down with you so you can still use the elevator:

    F4 .  .  .  .  .  
    F3 .  HG .  LG .  
    F2 E  .  HM .  .  
    F1 .  .  .  .  LM 

    At the first floor, grab the Lithium-compatible Microchip, which is safe because Microchips don't affect each other:

    F4 .  .  .  .  .  
    F3 .  HG .  LG .  
    F2 .  .  .  .  .  
    F1 E  .  HM .  LM 

    Bring both Microchips up one floor, where there is nothing to fry them:

    F4 .  .  .  .  .  
    F3 .  HG .  LG .  
    F2 E  .  HM .  LM 
    F1 .  .  .  .  .  

    Bring both Microchips up again to floor three, where they can be temporarily connected to their corresponding generators while the elevator recharges, preventing either of them from being fried:

    F4 .  .  .  .  .  
    F3 E  HG HM LG LM 
    F2 .  .  .  .  .  
    F1 .  .  .  .  .  

    Bring both Microchips to the fourth floor:

    F4 E  .  HM .  LM 
    F3 .  HG .  LG .  
    F2 .  .  .  .  .  
    F1 .  .  .  .  .  

    Leave the Lithium-compatible microchip on the fourth floor, but bring the Hydrogen-compatible one so you can still use the elevator; this is safe because although the Lithium Generator is on the destination floor, you can connect Hydrogen-compatible microchip to the Hydrogen Generator there:

    F4 .  .  .  .  LM 
    F3 E  HG HM LG .  
    F2 .  .  .  .  .  
    F1 .  .  .  .  .  

    Bring both Generators up to the fourth floor, which is safe because you can connect the Lithium-compatible Microchip to the Lithium Generator upon arrival:

    F4 E  HG .  LG LM 
    F3 .  .  HM .  .  
    F2 .  .  .  .  .  
    F1 .  .  .  .  .  

    Bring the Lithium Microchip with you to the third floor so you can use the elevator:

    F4 .  HG .  LG .  
    F3 E  .  HM .  LM 
    F2 .  .  .  .  .  
    F1 .  .  .  .  .  

    Bring both Microchips to the fourth floor:

    F4 E  HG HM LG LM 
    F3 .  .  .  .  .  
    F2 .  .  .  .  .  
    F1 .  .  .  .  .  

