# Trusted Carrier

Type-level unique "tags" used to mark types, thus providing a "chain of trust"  
for things like *safe* **unchecked** indexing, capability management and stuff like that.

### Disclaimer

This library is highly experimental and is intended for research purposes only.  
Soundness has not been proven, so I do NOT recommend using this library
for nothing more than research.

### References/Prior Art

- [Alexis Beingessner - Thesis](https://github.com/Gankra/thesis)
- [generativity](https://crates.io/crates/generativity)
- [unique-type](https://crates.io/crates/unique-type)
- [indexing](https://crates.io/crates/indexing)

- [variance](https://doc.rust-lang.org/nomicon/subtyping.html#variance)
- [sealed traits](https://internals.rust-lang.org/t/pre-rfc-sealed-traits/3108)

### License

MIT
