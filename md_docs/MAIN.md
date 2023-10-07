# Const_units

Const Units does automatic type-safe zero-cost dimensional and unit analysis.
Instead of just storing a value, a whole unit is stored as well.
Another benefit of using the [Unit](TODO) Datatype instead of just using a number is that printing will (optionally) display the unit and that conversions from one unit to another can be made by simply using into.

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