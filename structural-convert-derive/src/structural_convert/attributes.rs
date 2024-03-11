pub mod enum_variant;
pub mod from_container_attributes;
pub mod ident_as_literal_list;
pub mod into_container_attributes;
pub mod named_field;
pub mod try_from_container_attributes;
pub mod try_into_container_attributes;

use darling::FromAttributes;
use darling::FromDeriveInput;

use self::enum_variant::from_enum_variant_attributes::FromEnumVariantAttributes;
use self::enum_variant::into_enum_variant_attributes::IntoEnumVariantAttributes;
use self::enum_variant::try_from_enum_variant_attributes::TryFromEnumVariantAttributes;
use self::enum_variant::try_into_enum_variant_attributes::TryIntoEnumVariantAttributes;
use self::from_container_attributes::FromContainerAttributes;
use self::into_container_attributes::IntoContainerAttributes;
use self::named_field::from_field_named_attributes::FromFieldNamedAttributes;
use self::named_field::into_field_named_attributes::IntoFieldNamedAttributes;
use self::named_field::try_from_field_named_attributes::TryFromFieldNamedAttributes;
use self::named_field::try_into_field_named_attributes::TryIntoFieldNamedAttributes;
use self::try_from_container_attributes::TryFromContainerAttributes;
use self::try_into_container_attributes::TryIntoContainerAttributes;

#[derive(Debug, Default, FromDeriveInput)]
#[darling(default, attributes(convert))]
pub struct ContainerAttributes {
    #[darling(multiple)]
    pub into: Vec<IntoContainerAttributes>,
    #[darling(multiple)]
    pub from: Vec<FromContainerAttributes>,
    #[darling(multiple)]
    pub try_into: Vec<TryIntoContainerAttributes>,
    #[darling(multiple)]
    pub try_from: Vec<TryFromContainerAttributes>,
}

#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(attributes(convert))]
pub struct EnumVariantAttributes {
    #[darling(multiple)]
    pub from: Vec<FromEnumVariantAttributes>,
    #[darling(multiple)]
    pub into: Vec<IntoEnumVariantAttributes>,
    #[darling(multiple)]
    pub try_from: Vec<TryFromEnumVariantAttributes>,
    #[darling(multiple)]
    pub try_into: Vec<TryIntoEnumVariantAttributes>,
}

#[derive(Debug, Default, Clone, FromAttributes)]
#[darling(attributes(convert))]
pub struct FieldNamedAttributes {
    #[darling(multiple)]
    pub from: Vec<FromFieldNamedAttributes>,
    #[darling(multiple)]
    pub into: Vec<IntoFieldNamedAttributes>,
    #[darling(multiple)]
    pub try_from: Vec<TryFromFieldNamedAttributes>,
    #[darling(multiple)]
    pub try_into: Vec<TryIntoFieldNamedAttributes>,
}
