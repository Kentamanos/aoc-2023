# Bug
Thinking about this, I realize my proximity checking around numbers would potentially fail if there were 5 digit numbers in the input. My proximity checking code just looks at the beginning and end location of a number. This breaks down when 5 digit numbers are used (this does not occur in the input files).

For instance, this `*` would not be detected to be adjacent to `12345`. I would need (yet) another loop to fix this:
```
.......
.12345.
...*...
```

I might come back and fix this later. `😊`