use core::marker::ConstParamTy;

//TODO replace locale with this
//https://github.com/johnstonskj/rust-codes
#[derive(ConstParamTy, PartialEq, Eq)]
struct Locale;

#[derive(ConstParamTy, PartialEq, Eq)]
pub struct PrintingStyle {
    /// How to represent the number
    number: NumberRepOuter,
    /// Used for translation
    locale: Locale,
    /// How to display the Unit
    unit_display: UQDisplay,
    /// How to display the Quantity
    quantity_display: UQDisplay,
    /// Wether to display if the unit is initialized
    display_init: bool,
    /// Wether/How to display the Prefix´s numerical representation too
    display_prefix: PrefixRep, //TODO turn this into an Option as soon as Option<T> implements/derives ConstParamTy
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]

pub struct UQDisplay {
    u_name_version: NameVersion,
    fractional: FractionalDisplay,
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum NameVersion {
    Abbreviation,
    FullName,
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum FractionalDisplay {
    /// usually in combination with [NameVersion::FullName]:
    /// 2 meters per second squared
    /// 1 kilogram-meter per second squared = 1 Newton; 2 kilogram-meters per second squared = 2 Newton
    /// 1 second squared per kilogram and meter to the fith power
    /// 1 millimeter per second
    Sentence,
    /// usually in combination with [NameVersion::Abbreviation]:
    /// 2 m/s²
    /// 1 kg*m/s² = 1 N; 2 kg*m/s²
    /// 1 s²/(kg*m⁵)
    /// 1 mm/s
    Fraction,
}

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum PrefixRep {
    /// Does not display the numerical representation at all
    /// TODO: turn this into an Option... see above
    None,
    /// e.g.    (false):   km[1000]/ms[1/1000]
    /// e.g.    (true):    km[1000/1]/ms[1/1000]
    /// =>      true ->    force denominator
    /// =>      false ->   do not display denominator if it is 1
    Fraction(bool),
    /// e.g.:   km[10^3]/ms[10^-3]
    Scientific(ScientificRep),
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct ScientificRep {
    base: BaseRep,
    /// e.g. (false): km[10^3]/ms[10^-3]
    /// e.g. (true): km[1*10^3]/ms[1*10^-3]
    force_one: bool,
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum BaseRep {
    /// e.g.:   km[E3]/ms[E-3]
    E,
    /// e.g.:   km[10^3]/ms[10^-3]
    Base10,
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub enum NumberRep {
    /// Written out with [language specific formatting](ttps://crates.io/crates/num-format)
    /// e.g. 1000000 or 1.1 or 0.078
    Full,
    /// Scientific Notation todo!() insert ISO number here (cant find it....)
    Scientific(ScientificRep),
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct NumberRepOuter {
    inner: NumberRep,
    /// If not None -> add a ≈ (by default)
    rounding: RoundingOuter, //TODO turn this into an Option as soon as Option<T> implements/derives ConstParamTy
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct RoundingOuter {
    inner: Rounding,
    add_approx_symbol: bool,
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
/// If not None -> add a ≈
pub enum Rounding {
    /// TODO turn [RoundingOuter] into an Option... see above
    None,
    /// Round to a whole number
    /// e.g. 420,69 -> 421
    Integer,
    /// Round to n significant figures base 10
    /// bool: show "(n)" immediately after the number?
    /// i8: 3 => 420,69 ->  420,7
    SignificatFiguresBase10(i8, bool),
    /// Round to n significant figures base 2
    /// bool: show "(n)" immediately after the number?
    /// THIS IS JUST AN IDEA CURRENTLY !unimplemented()
    SignificantFiguresBits(i8, bool),
}

pub const DEFAULT_PRINTINGSTYLE: PrintingStyle = {
    let uqdisplay = UQDisplay {
        u_name_version: NameVersion::Abbreviation,
        fractional: FractionalDisplay::Fraction,
    };
    PrintingStyle {
        number: NumberRepOuter {
            inner: NumberRep::Scientific(ScientificRep {
                base: BaseRep::Base10,
                force_one: false,
            }),
            rounding: RoundingOuter {
                inner: Rounding::None,
                /// This feels wierd and will therefore be fixed [Rounding]
                add_approx_symbol: false,
            },
        },
        locale: Locale,
        unit_display: uqdisplay,
        quantity_display: uqdisplay,
        display_init: false,
        display_prefix: PrefixRep::None,
    }
};
