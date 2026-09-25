# rlox
A full implementation of [Lox](https://craftinginterpreters.com/contents.html) in Rust, with added arrays.
Note: This is just the interpreter, not the VM.

## Arrays
I added arrays to my Lox implementation. Below is a demo:
```
var array = [1, 2, 3];

print array[1]; // 2
array[2] = 1;
print array[2]; // 1

print len(array); // 3

push(array, 5);
print array; // [1, 2, 1, 5] 
```

In conclusion, I added array literals, indexing, and several std methods for fetching length and pushing to end.
The std methods (`len` and `push`) additionally work on strings.
```
var string = "Hello";
print push(string, " world!"); // Hello world!

print string; // Hello (strings are immutable)
print len(string); // 5
```

Something I would add in the future is for-each loops (which would involve iterator work) and hashmaps (dicts).