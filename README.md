# Amnis

A minimal streaming RPC language, which is Some-Sort-of-Shell-Script (SSoSS).

## Progress

I just wrote this `README.md`. I'll try to write a Rust trait.

## Escape Sequence

### Input

* `\` for escaping.
* `(` and `)` for parameter list.
* ` ` for word spacing.
* Line feed for statement spacing.
* `#` for comments.
* `_` for discarded values.

Escape them with `\*` (`\n` for the line feed).

`bytes(n) b`, then in the next line, read `n` bytes to `b`, ignoring escapes.

### Output

* Line feed for output partioning.
* `#` for error messages.
* `_<length>_` for bytes in the next line, ignoring escapes.

Only the first byte needs escaping.

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
str(Hello,\ World!) s   # Create a string from a name.
print(s)                # Finally, hello world!
```

You can't create functions on the client side.
If you want more features, build a compiler.

## TV-like concurrency

Inspired by Zig async and TVs.

```
fn(x) y
mux()               # Multiplexing shares variables.
_ play() chan1      # Create channels.
_ play() chan2
chan1 fn(x1) y1
chan2 fn(x2) y2
chan1 stop()        # Only possible to stop locally.
chan2 stop()
_ demux()           # Wait for all channels to stop.
anotherFn(y y1 y2)
```

Use `pause()` and `resume()` to sync.

```
mux()
_ play() chan1
_ play() chan2
_ play() finish
finish pause()
chan1 work1()
chan1 resume(finish)
finish pause()
chan2 work2()
chan2 resume(finish)
finish str(done) s
finish print(s)
finish stop()
_ demux()
```

## Iterator

When a function detects an iterator version of input,
it runs multiple times then collect the result to a list.
In async context, use `iterPara()` to parallelize.

The use of iterator is strongly discouraged.
If you need list operation, you may implement server side.

Struct is not supported, use the SoA pattern.

```
i32Array(1 2 3) a
iter(a) i
print(i)
```

The above is just a fancier below.

```
i32(1) i
print(i)
i32(2) i
print(i)
i32(3) i
print(i)
```

Yes, there is a range that takes less space.

```
i32Range(1 3) r
```

All three iterate in the same way.

```
i32RangeSecond(1 3 9) r1
i32RangeStep(1 2 9) r2
i32Array(1 3 5 7 9) a
```

## Debug

Use `debug` to print state. To print gas metrics.

```
str(gas) g
strArray(g) a
debug(a)
```

## Application Scenario

1. Functions are expensive, mostly database queries.
2. The memory of the interpreter is small and expensive.
3. Interact with a query frontend at the edge.
4. Parsing time is bounded, so no code generation syntax.
