use super::{dim_type::System, SystemDim};
use crate::global_types::{quantity::Quantity, QName, SiExtended};

impl Quantity {
    pub const fn from_name(name: QName, dim_type: System) -> Self {
        match dim_type {
            System::SiExtended => match name {
                QName::Velocity => Self {
                    name,
                    dimensions: SystemDim::SiExtended(SiExtended { a: 0 }),
                },
                QName::Length => Self {
                    name,
                    dimensions: SystemDim::SiExtended(SiExtended { a: 0 }),
                },
                QName::Time => Self {
                    name,
                    dimensions: SystemDim::SiExtended(SiExtended { a: 0 }),
                },
            },
        }
    }
}
