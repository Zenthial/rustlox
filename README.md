# RustLox

An implementation of the language lox, from Robert Nystrom's book 'Crafting Interpreters'.

This implementation is done in Rust, while the original implementation in the book is done in C.

There are two reasons for this, though three if you count the fact that I just like Rust. The two core reasons are Enums and Memory.

Writing code in C, enums do not feel powerful enough. Especially after using Rust, not having the ability to store data inside enums feels bad.

When it comes to memory, I would prefer to not have to track down memory leaks when going off on my own paths, without the guidance of the book.

Many of the implementations written in the book may be modified to become more "rusty".