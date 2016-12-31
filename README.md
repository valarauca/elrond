Elrond
---
[Docs](https://valarauca.github.io/elrond/elrond/index.html)

This is an embryonic Elf Parser written in NOM. It is not feature complete. It requires a lot more testing this is just a cleaned up version I can throw on `crates.io` to start testing with.

If you want to use this:

```
[dependencies]
elrond = 0.0.1
```

Non-standard values will currently result in a panic.

TODO:

- [X] Get it to compile
- [ ] Test against real ELF files
- [ ] Write useful tests
- [ ] Get Note/Dyn tables working
- [ ] Expose string table in default structure
- [ ] Make structures more ergonomic to work with

####Code Example:

```rust
extern crate elrond;
use elrond::Elf;


/*
 * Read an Elf File
 */
let mut v = Vec::with_capacity(4096);
let _ = my_file.read_to_end(v.as_mut_slice())?;
let elf = match Elf::parse(v.as_slice()) {
  Option::Some(x) => x,
  Option::None => panic!("\n\nCould not parse elf file\n\n")
};


```
