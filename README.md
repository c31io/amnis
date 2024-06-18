# Amnis

A minimal streaming RPC language, which is Some-Sort-of-Shell-Script (SSoSS).

## Progress

I just wrote this `README.md`.

## Escape Sequence

### Input

* `\` for escaping.
* `(` and `)` for parameter list.
* ` ` for word spacing.
* Line feed for statement spacing.
* `#` for comments.
* `_` for discarded values.

Escape them with `\*` (`\n` for the line feed).

### Output

* Line feed for output partioning.
* `#` for error messages.

## Syntax

`fn(in1 in2) out1 out2`

## In a Nutshell

```
# Set the version
version(1)          # The server function version.
myFunction()        # Your function.

# Variable
i32(1) x            # Initialize x with i32 value 1.
f64Array(1 2 3) fa  # Create fa, a f64 array [1,2,3].
drop(a)             # Drop fa to free some space.

# Control flow
eq(x x) b           # b is true.
goto(labelA)        # Go to line "label(labelA)".
exit()              # An unreachable exit.
label(labelA)       # Define a label.
ifgo(b labelB)      # If b is true, then goto labelB.
unreachable()       # An unreachable assertion.
label(labelB)
clear()             # You can't jump up anymore.
not(b) c            # b is true, so c is false.
ifel(c laC laD)
label(laC)          # If c is true, start here.
unreachable()
label(laD)          # If c is false, start here.
str(Hello,\ World!) s   # Create a string.
print(s)                # Finally, hello world!
```

You can't create functions on the client side.
If you want more features, build a compiler.