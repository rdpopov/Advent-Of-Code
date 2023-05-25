# Day 24

Little bit slow in generating all the groups but it is simple at least, which
might make it faster.

n distinct groups don't need to be generated, not sure why that is the case.
Maybe the input is specially made for this

So I just need to generate all the combinations and do the minimum on length and
product

Sum the numbers , div by the number of sums to find a singular sum then 
generate the possible sums with a generator function, 

Each level either includes the number in the sum or doesn't(by subtracting it
from current sum),
if the sum on entry is 0 we return the current chain.

Iterate over them and minimize over length and value of product
