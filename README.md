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
