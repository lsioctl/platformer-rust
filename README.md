# platformer-rust

An attempt to make a platformer game from scratch with assets and code base from this
Monogame example:

https://github.com/MonoGame/MonoGame.Samples/tree/3.8.1/Platformer2D

My goal is to keep practicing Rust and having fun with Video Game programming.

Bevvy looked promising, but is higher level AFAIK.

Another inspiration will be this article, with OCaml/Raylib:

https://dev.to/sheosi/making-a-game-engine-with-ecs-and-in-ocaml-2oma

## raylib installation

- I just ditched windows as we havec cmake dependencies, I think the target would be using `cross`
  as recommended in Raylib's doc
- On Linux, a bunch of dependencies also:

```bash
apt-get install cmake libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev -y
```

## raylib limitations

It says it supports Direct3D, Metal, through ANGLE, but support may be very experimental
