# Pong Example
This is a test repository for the game engine EDAME that I am developing. It is simply a clone of pong.

## Ideas for EDAME
- There is a StateSpace.
- The StateSpace is updated based on inputs and can generate outputs.
- The update rules are as follows:
```math
\frac{\mathrm{d} s}{\mathrm{dt}} = B^{-1}(i)A(s)\frac{\mathrm{d} i}{\mathrm{dt}}
o = B(i)s + A(s)i
```
