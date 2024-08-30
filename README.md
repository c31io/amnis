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

`_ bytes(n) b`, then in the next line, read `n` bytes to `b`, ignoring escapes.

### Output

```
message size, including everything below LF
channel name LF
line number LF
message type (can be FIN) LF
message content LF
```

## Syntax

`chan fn(in1 in2) out1 out2`

## In a Nutshell

```
# Variable
_ i32(1) x              # Initialize x with i32 value 1.
_ f64Array(1 2 3) fa    # Create fa, a f64 array [1,2,3].
_ drop(a)               # Drop fa to free some space.

# Control flow
_ eq(x x) b             # b is true.
_ goto(labelA)          # Go to line "label(labelA)".
_ exit()                # An unreachable exit.
_ label(labelA)         # Define a label.
_ ifgo(b labelB)        # If b is true, then goto labelB.
_ unreachable()         # An unreachable assertion.
_ label(labelB)
_ clear()               # You can't jump up anymore.
_ not(b) c              # b is true, so c is false.
_ ifel(c laC laD)
_ label(laC)            # If c is true, start here.
_ unreachable()
_ label(laD)            # If c is false, start here.
_ str(Hello,\ World!) s # Create a string from a name.
_ print(s)              # Finally, hello world!
_ stop()                # If not send, request will stay alive.
```

You can't create functions on the client side.
If you want more features, build a compiler.

## TV-like & Zig-like concurrency

### TV-like handle-join parallelism.

```
_ play() chan1      # Create channels.
_ play() chan2
chan1 fn1(x1) y1
chan2 fn2(x2) y2
chan1 stop()        # Only possible to stop locally.
chan2 stop()        # Block until the channel finish.
_ fn3(y1 y2) z
_ stop()
```

Intuitive but might cause heavy blockage.

### Zig-like pause-resume concurrency.

Use `pause()` and `resume()` to sync.

```
_ play() chan1
_ play() chan2
_ pause()
chan1 work1()
chan1 resume(_)
_ pause()
chan2 work2()
chan2 resume(_)
_ str(done) s
_ print(s)
chan1 stop()
chan2 stop()
_ stop()
```

Channel suspends when pauses > resumes.

### Async Bytes I/O

Big payload should use http, not Amnis.

Async input is sent in chunk upload functions.

Async output arrives in chunks, with the same output format.

### Why not Async-Await?

No color in Amnis, implement color client side.

## Application Scenario

1. Functions are expensive, mostly database queries.
2. The memory of the interpreter is small and expensive.
3. Interact with a query frontend at the edge.
4. Parsing time is bounded, so no code generation syntax.

## Wire Format

Layered on top of reliable duplex protocols, e.g. WS, TCP, and QUIC.

### Input

i32 channel - i32 function - ( fixed-size input | ( u64 size - [u8; size] ) ) 

channel: 1 is played on start.

function: < 0 is buit-in. = 0 prints source code URL. > 0 is user-defined.

### Output

i32 channel - i32 line - u64 size - [u8; size]

channel: +n is output, and -n is error message, channel 0 is reserved.

line < 0 means the last chunk of |line|.
