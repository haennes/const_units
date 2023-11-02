# Const_units

Const Units does automatic type-safe zero-cost dimensional and unit analysis.
Instead of just storing a value, a whole unit is stored as well.
Another benefit of using the [Unit](TODO) Datatype instead of just using a number is that printing will (optionally) display the unit and that conversions from one unit to another can be made by simply using into.
This library will support constants as well. [Physical Constants](#constants) simply are a unit.
## Usage

```rust

uuse!(millimeter_u16, second_u16)


fn main(){
    let length = millimeter_u16 * 8;
    let time = second_u16 * 2;
    let v = length / time;
    let a = acceleration(v, time);
    // let error = length + time; // error[E0308]: mismatched types
}

fn acceleration<const UV, const UT, DT: QuantityDataTraits>(velocity: Velocity<UV, DT>, time: Time<UT, DT> -> Acceleration<{UV.div(UT)}, DT>{
    velocity / time
}

```

## Units

A Unit is represented like this:
```rust
pub struct Unit<
    StorageDt: QuantityDataTraits,
    /// Identifier
    const UNIT: UName,
    /// Generic of Units of the same Quantity
    const QUANTITY: Quantity,
    /// code deduplication and generics
    const PREFIX: Prefix = { Prefix::from(PName::None) },
    const INITIALIZED: bool = false,
    const PRINTINGSTYLE: PrintingStyle = { DEFAULT_PRINTINGSTYLE },
> {
    value: StorageDt,
}
```
## Prefix
```rust
pub struct Prefix {
    /// an enum of all Prefix Names. Generated at compile-time
    pub name: PName, 
    pub factor: Factor,
}


pub enum Factor {
    Ratio(RatioConst),
    Float(F64),
}
```

## Quantities
```rust
pub enum Quantity {
    Simple(BaseQuantity),
    Complex(CVec<BaseQuantity, COMPLEX_LEN_MAX>),
}
```


## constants
Despite constants generally being precieved as fundamentally differnt to units this library treats them is if they were normal units. 

Take the gravitational constant `G` as an example. Its value is (approximately equal)[https://en.wikipedia.org/wiki/Gravitational_constant] to `6.67430 * 10^-11 N*m^2*kg^-2` or `43009172706 * 10^-3 pc*M^-1*(km/s)^2`. Therefore we can treat it as a unit that when converted to the unit `N*m^2*kg^-2` just has to be multiplied by `6.67430 * 10^-11` or when converted to the unit `pc*M^-1*(km/s)^2` just has to be multiplied by `43009172706 * 10^-3`. Hence we declare `G` in `G.toml` like this:
```toml
symbol = "G"
description = "gravitational constant" #optional
derive_prefixes = [] #overwrites system´s config and optional

[names]
Eng = "gravitational constant"

[conversions]
# 1 ft = ?
Newtonmeter_per_kilogram_squared = 6.67430E-11
"pc(km_per_s)_squared_per_M" = 43009172706E-3 
```
Even adding aditional data about the accuracy is possible