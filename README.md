# AoC2020
Advent of Code 2020

## Day 1
Task: Given a list of numbers, find two (for part two three) numbers thast add up to 2020 and return their product.

Quite a simple Task.
Since the list of numbers is decently small I took the lazy approach and just did two (or three) nested for loops.
Works fine, not very optimal.
Wow, Day 1 and I'm already lazy :D

Edit: Ok, stop lazy. I added an optimal solution for Part One at least.

## Day 2
Task: Given a list of passwords and their restrictionc, count the valid passwords.

All in all, rather straightforward.
Had a small headache with `str` ownership, but in the end wrapping it in a `String` solved the issue.
Oh, and I misread the required XOR for a OR at first.
Curiously, it doesn't appear like there exists a boolean XOR operator in rust.

This time, there's no complexity analysis.
Any algorithm should solve this problem in linear time.
One thing I could possibly imporve is change to a stream approach for puzzles like this.
I don't need to read the entire file at once.

## Day 3
Task: Given a horizontally tiling map, count the trees you encounter on the slope down.

Again, rather straightforward.
3 steps to the right, wrap around if needed, 1 step down.
I decided to implement the `FromStr` trait for my lines now.
My plan with this years AoC was to learn rust a little bit better, so this is a nice step.

For part two, I made my function a little more generic.
Little oopsie happened when the multiplication resulted in an overflow.
Guess 32 bits weren't enough.
And then i copied it wrong and had to wait a minute before submitting again.

## Day 4
I spent way too much time on this:
The Regex-Sledgehammer (for part one):

```
(?=(?:.(?!\n\n))+.byr)
(?=(?:.(?!\n\n))+.iyr)
(?=(?:.(?!\n\n))+.eyr)
(?=(?:.(?!\n\n))+.hgt)
(?=(?:.(?!\n\n))+.hcl)
(?=(?:.(?!\n\n))+.ecl)
(?=(?:.(?!\n\n))+.pid)
.*?\n\n
```

For some reason I got an off by one error and got 209 matches instead of 208.
Also my rust regex library doesn't support lookahead. Soooo...
I wrote the one-liner myself :D

Part two just seemed like a lot of effort.
Though it probably integrates nicely with the regex....
Anyway, can't be bothered.

My actual solution for part two is also off by one.
I don't know which one is wrong, and frankly, I don't care.
This part two to me fealt more like work (being foced to doing tons of checks) rather than fun problem solving.

So much for not being lazy :D

Edit: I talked to a friend and I probably wrongly accepted a 10-digit pid.
Either way, least favorite one so far, I won't bother fixing this (even though it's easy).

## Day 5
This one was fun!
And it was fun, cause there was a super easy clean solution to this.
Essentially, the seats are just weirdly encoded binary numbers.
Every `F` and `L` encodes a `0` and every `B` and `R` encodes a `1`.
Parse that, and return the highest number.

Part 2 was also straightforward:
All numbers are consecutive, find the first number that got skipped.

I have a decently clean solution, can't complain, fun puzzle!
Tomorrow I won't be home, so I'll do 2 days at once on monday!

## Day 6
Wasn't home for this one, so I'm a day late.
I liked it!
Thanks to the power of HashSets and unions/intersections it was rather straightforward again.

## Day 7
Wow!
That one actually took me a bit.
I knew what I wanted to do, bu I couldn't do it.
Parsing the input was another one-liner (with some regex this time), but I didn't want to do recursion.
Also I really wanted to avoid loops, although I assume those don't exist in the input.
I'm quite happy with my solution, though I didn't bother with efficiency for part two at all.

## Day 8
Part one was pretty basic again.
I used a fancy map to keep track of the instructions already executed and can simply abort once I reach one.

Part two was a bit more tricky.
To be fair, brute forcing is viable and also what I ended up doing, but I was thinking about more efficient solutions.
For example, backtracking from the last instruction to see where the chain breaks.
But there were so many issues with that thought.

I also helped my friend get started in rust for Advent of Code.
That was a lot of fun, even though there were some struggles with installing the toolchain.

## Day 9
I loved this one!
I spent most of my time trying to figure out an overly generic way of validating the preamble,
before deciding to ditch all of that.
Generics are fun, but I don't have enough time to spent an entire day on fixing my generics.

Part two was great too! I went for the optimal O(n) solution.
Learned some cool new iterator functions like `scan`.
I enjoyed having a bit of a brain challenge and not just a coding challenge though.

## Day 10
Fun stuff!
First time this year that brute force isn't a solution.
Some form of dynamic programming is required.
Easiest solution is probably just using a HashMap as a cache.
I just used an array, which is probably a tiny bit faster.

I want more like this!

## Day 11
Well...
This one took me a while.
Not because it was hard, but because I tried to implement a proper grid data structure with all iterator traits implemented etc.
I gave up after a day, and the current solution is decent.
The `Vec<Vec<char>>` provides iterators and maps for me to use anyway.
Maybe I'll try again another day.