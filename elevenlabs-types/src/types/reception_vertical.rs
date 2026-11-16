pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReceptionVertical {
    SalonBarbershop,
    NailBeauty,
    Spa,
    Medical,
    Veterinary,
    Fitness,
    HomeServices,
    Photography,
    Consulting,
    Restaurant,
    Dental,
    AutoService,
    Tutoring,
    Legal,
    RealEstate,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReceptionVertical {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SalonBarbershop => serializer.serialize_str("salon_barbershop"),
            Self::NailBeauty => serializer.serialize_str("nail_beauty"),
            Self::Spa => serializer.serialize_str("spa"),
            Self::Medical => serializer.serialize_str("medical"),
            Self::Veterinary => serializer.serialize_str("veterinary"),
            Self::Fitness => serializer.serialize_str("fitness"),
            Self::HomeServices => serializer.serialize_str("home_services"),
            Self::Photography => serializer.serialize_str("photography"),
            Self::Consulting => serializer.serialize_str("consulting"),
            Self::Restaurant => serializer.serialize_str("restaurant"),
            Self::Dental => serializer.serialize_str("dental"),
            Self::AutoService => serializer.serialize_str("auto_service"),
            Self::Tutoring => serializer.serialize_str("tutoring"),
            Self::Legal => serializer.serialize_str("legal"),
            Self::RealEstate => serializer.serialize_str("real_estate"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReceptionVertical {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "salon_barbershop" => Ok(Self::SalonBarbershop),
            "nail_beauty" => Ok(Self::NailBeauty),
            "spa" => Ok(Self::Spa),
            "medical" => Ok(Self::Medical),
            "veterinary" => Ok(Self::Veterinary),
            "fitness" => Ok(Self::Fitness),
            "home_services" => Ok(Self::HomeServices),
            "photography" => Ok(Self::Photography),
            "consulting" => Ok(Self::Consulting),
            "restaurant" => Ok(Self::Restaurant),
            "dental" => Ok(Self::Dental),
            "auto_service" => Ok(Self::AutoService),
            "tutoring" => Ok(Self::Tutoring),
            "legal" => Ok(Self::Legal),
            "real_estate" => Ok(Self::RealEstate),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReceptionVertical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SalonBarbershop => write!(f, "salon_barbershop"),
            Self::NailBeauty => write!(f, "nail_beauty"),
            Self::Spa => write!(f, "spa"),
            Self::Medical => write!(f, "medical"),
            Self::Veterinary => write!(f, "veterinary"),
            Self::Fitness => write!(f, "fitness"),
            Self::HomeServices => write!(f, "home_services"),
            Self::Photography => write!(f, "photography"),
            Self::Consulting => write!(f, "consulting"),
            Self::Restaurant => write!(f, "restaurant"),
            Self::Dental => write!(f, "dental"),
            Self::AutoService => write!(f, "auto_service"),
            Self::Tutoring => write!(f, "tutoring"),
            Self::Legal => write!(f, "legal"),
            Self::RealEstate => write!(f, "real_estate"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
