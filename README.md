# Pico 3D

## Introduction

I am building this project to demonstrate how I build a micro 3D graphics library for the raspberry pi pico.

One day while surfing youtube I came accross this amazing video by Tsoding [Demystifies 3D Graphics](https://youtu.be/qjWkNZ0SXfo?si=XWEg81_SU7X2LW7I)
in which he breaks down how graphics apis like Vulkan and Metal actually work under the hood. Of cource they contain
a load of more features but to their core the way they render graphics is the same.

So I thought to myself that it's a great learning opportunity to build one myself, however to make things harder rather
than using HTML and JS (as used by Tsoding in his video), I took the challenge to build it for a raspberry pi pico.

Combining the bare metal nature of the pico and the amazing memory safety guaranteed by rust, I feel like it would make
up for a great learning experience.

## More To Know

I have built the library on top of [_Embedded Graphics_](https://crates.io/crates/embedded-graphics) crate, which is a
popular 2D graphics library.