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

## Animation reference

https://www.raylib.com/examples/textures/loader.html?name=textures_sprite_anim

## Keyboard input

Coming from Raylib examples in C I was stuck with ffi and unsafe calls, this documentation
showed me how to use proper Rust calls:

https://pls.plaureano.com/blog/files/Example-code-2-rust.php

## Parallax reference

https://www.raylib.com/examples/textures/loader.html?name=textures_background_scrolling


## Issue to investigate (or not ...)

Got a nasty issue while build raylib-rs with cargo and gcc:

```bash
--- stderr
  binding/../raylib/src/raylib.h:88:10: fatal error: 'stdarg.h' file not found
```

`stdarg.h` was here:

```bash
# dpkg -S /usr/lib/gcc/x86_64-linux-gnu/13/include/stdarg.h 
libgcc-13-dev:amd64: /usr/lib/gcc/x86_64-linux-gnu/13/include/stdarg.h
```

When launching gcc manually as cargo seemed to do, no issue ...

as I noticed a SO post saying to install clang, as I think cargo and rust are
related a lot to LLVM, and this cargo message:

```bash
thread 'main' panicked at /home/lsioctl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/raylib-sys-5.5.1/build.rs:303:39:
  Unable to generate bindings: ClangDiagnostic("binding/../raylib/src/raylib.h:88:10: fatal error: 'stdarg.h' file not found\n")
```

I gave up, installed clang ... and it worked :(


