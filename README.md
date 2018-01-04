Elrond

[Docs](https://valarauca.github.io/elrond/elrond/index.html)

---

### To use

Add this to your `Cargo.toml`

```
[dependencies]
elrond = "1.0.0"
```

---

### Code Example

```rust
extern crate elrond;
use elrond::prelude::*;

let mut v = Vec::with_capacity(4096);
let _ = my_file.read_to_end(v.as_mut_slice())?;
let elf = match elrond::parse_elf(v.as_slice()) {
	Ok(x) => x,
	Err(e) => panic!("Could not read elf file {:?}", e)
};

```

This crate is fairly feature complete and standard conformant. 

It doesn't support all the GNU extensions which you'll likely encounter in a modern Linux or
OSX binary. But **MOST** of what you want is here.

Also the some standards documents disagree on if the `HiOS` and `LoOS` are an inclusive range,
or just markers. I'm just treating them as unique markers. Incorrect values are passed as `Unknown`

---

This is still a work in progress but a lot of progress has been made

