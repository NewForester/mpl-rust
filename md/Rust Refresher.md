<!-- mpl-rust by NewForester:  programming notes on and examples in Rust -->

# Rust Refresher Notes

Anyone can remember things they use everyday but it is not everyday one reinstalls a programming language.
No matter how well one knows a programming language, some details fade if one spends any length of time using another.

Below is not a Rust tutorial but quick refresher notes.
Hopefully just enough to bring back all you do know that has slipped just beyond the instant recall threshold.

Do not forget to update these notes with what you have learnt since they were written.

## Overview

[Rust](https://en.wikipedia.org/wiki/Rust_(programming_language))
is modern systems programming language sponsored by Mozilla Research that first appeared in 2010.

Rust is a compiled language that follows the structured / imperative paradigm but
also supports generic and functional programming.
The language is strongly and statically typed but features type inference and structural typing.
Memory management does not include garbage collection.

Rust features concurrency but memory safety without a performance penalty is its defining characteristic.

The current stable release is 1.25.0 (released March 29 2018).

Rust, its compiler and tools are free and open source and available under either Apache or MIT licences.

Rust seeks to improve on C++ with a feature set that emphasises safety, control of memory layout and concurrency.
Together with features to support the creation and maintenance of 'boundaries' these support "programming in the large".

Rust is not a class-based object oriented language but provides 'implementations', without inheritance, and
'traits' (along the lines of Haskell) that provide inheritance and polymorphism.

The use of 'traits' to implement polymorphism
allows structural type checking at compile time but still involves separate, optimised, code for each instantiation.

Rust has a package and build automation system named Cargo.

The official web-site is [https://www.rust-lang.org/](www.rust-lang.org).

---

*mpl-rust* by NewForester.
Notes licensed under a Creative Commons Attribution-ShareAlike 4.0 International Licence.

<!-- EOF -->
