//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use std::collections::HashMap;

use serde::Serialize;

pub use type_description_derive::TypeDescription;

/// Generic config that represents what kind of config a plugin wishes to accept
#[derive(Debug, Serialize, PartialEq)]
pub struct TypeDescription {
    name: String,
    kind: TypeKind,
    doc: Option<&'static str>,
}

impl TypeDescription {
    /// Construct a new generic config explanation
    #[must_use]
    pub fn new(name: String, kind: TypeKind, doc: Option<&'static str>) -> Self {
        Self { name, kind, doc }
    }

    /// Get a reference to the config's documentation.
    #[must_use]
    pub fn doc(&self) -> Option<&'static str> {
        self.doc
    }

    /// Get a reference to the config's kind.
    #[must_use]
    pub fn kind(&self) -> &TypeKind {
        &self.kind
    }

    /// Set or replace the documentation of this [`TypeDescription`]
    #[must_use]
    pub fn with_doc(mut self, doc: Option<&'static str>) -> Self {
        self.doc = doc;
        self
    }

    /// Get the config's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// How an enum is represented
#[derive(Debug, Serialize, PartialEq)]
pub enum EnumVariantRepresentation {
    /// The enum is represented by a string
    ///
    /// This is the case with unit variants for example
    String(&'static str),
    /// The enum is represented by the value presented here
    Wrapped(Box<TypeDescription>),
}

/// The kind of enum tagging used by the [`TypeKind`]
#[derive(Debug, Serialize, PartialEq)]
pub enum TypeEnumKind {
    /// An internal tag with the given tag name
    Tagged(&'static str),
    /// An untagged enum variant
    Untagged,
}

/// The specific kind a [`TypeDescription`] represents
#[derive(Debug, Serialize, PartialEq)]
pub enum TypeKind {
    /// Type represents a boolean `true`/`false`
    Bool,

    /// Type represents an integer `1, 10, 200, 10_000, ...`
    ///
    /// # Note
    ///
    /// The maximum value that can be represented is between [`i64::MIN`] and [`i64::MAX`]
    Integer,

    /// Type represents a floating point value `1.0, 20.235, 3.1419`
    ///
    /// # Note
    /// Integers are also accepted and converted to their floating point variant
    ///
    /// The maximum value that can be represented is between [`f64::MIN`] and [`f64::MAX`]
    Float,

    /// Type represents a string
    String,

    /// Wrap another config
    ///
    /// This is particularly useful if you want to restrict another kind. The common example is a
    /// `Port` config object which is represented as a `u16` but with an explanation of what it is
    /// meant to represent.
    Wrapped(Box<TypeDescription>),

    /// Type represents an array of values of the given [`TypeKind`]
    Array(Box<TypeDescription>),

    /// Type represents a hashmap of named configurations of the same type
    ///
    /// # Note
    ///
    /// The key is always a [`String`] so this only holds the value config
    HashMap(Box<TypeDescription>),

    /// Type represents a map of different configurations
    ///
    /// The tuple represent `(field_name, documentation, config_description)`
    Struct(Vec<(&'static str, Option<&'static str>, TypeDescription)>),

    /// Type represents multiple choice of configurations
    Enum(
        TypeEnumKind,
        Vec<(
            &'static str,
            Option<&'static str>,
            EnumVariantRepresentation,
        )>,
    ),
}

/// Turn a plugin configuration into a [`TypeDescription`] object
///
/// Plugin authors are expected to implement this for their configurations to give users
pub trait AsTypeDescription {
    /// Get a [`TypeDescription`] object from the type
    fn as_type_description() -> TypeDescription;
}

impl<T: AsTypeDescription> AsTypeDescription for Option<T> {
    fn as_type_description() -> TypeDescription {
        TypeDescription::new(
            format!("An optional '{}'", T::as_type_description().name()),
            TypeKind::Wrapped(Box::new(T::as_type_description())),
            None,
        )
    }
}

impl<T: AsTypeDescription> AsTypeDescription for Vec<T> {
    fn as_type_description() -> TypeDescription {
        TypeDescription::new(
            format!("Array of '{}'s", T::as_type_description().name()),
            TypeKind::Array(Box::new(T::as_type_description())),
            None,
        )
    }
}

impl<V: AsTypeDescription> AsTypeDescription for HashMap<String, V> {
    fn as_type_description() -> TypeDescription {
        TypeDescription::new(
            format!("Table of '{}'s", V::as_type_description().name()),
            TypeKind::HashMap(Box::new(V::as_type_description())),
            None,
        )
    }
}

impl<V: AsTypeDescription> AsTypeDescription for HashMap<std::path::PathBuf, V> {
    fn as_type_description() -> TypeDescription {
        TypeDescription::new(
            format!("Table of '{}'s", V::as_type_description().name()),
            TypeKind::HashMap(Box::new(V::as_type_description())),
            None,
        )
    }
}

macro_rules! impl_config_kind {
    ($kind:expr; $name:expr; $doc:expr => $($typ:ty),+) => {
        $(
            impl AsTypeDescription for $typ {
                fn as_type_description() -> TypeDescription {
                    TypeDescription::new({$name}.into(), $kind, Some($doc))
                }
            }
        )+
    };
}

impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 64 bits" => i64);
impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 64 bits that cannot be zero" => std::num::NonZeroI64);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 64 bits" => u64);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 64 bits that cannot be zero" => std::num::NonZeroU64);

impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 32 bits" => i32);
impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 32 bits that cannot be zero" => std::num::NonZeroI32);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 32 bits" => u32);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 32 bits that cannot be zero" => std::num::NonZeroU32);

impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 16 bits" => i16);
impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 16 bits that cannot be zero" => std::num::NonZeroI16);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 16 bits" => u16);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 16 bits that cannot be zero" => std::num::NonZeroU16);

impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 8 bits" => i8);
impl_config_kind!(TypeKind::Integer; "Integer"; "A signed integer with 8 bits that cannot be zero" => std::num::NonZeroI8);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 8 bits" => u8);
impl_config_kind!(TypeKind::Integer; "Integer"; "An unsigned integer with 8 bits that cannot be zero" => std::num::NonZeroU8);

impl_config_kind!(TypeKind::Float; "Float"; "A floating point value with 64 bits" => f64);
impl_config_kind!(TypeKind::Float; "Float"; "A floating point value with 32 bits" => f32);

impl_config_kind!(TypeKind::Bool; "Boolean"; "A boolean" => bool);
impl_config_kind!(TypeKind::String; "String"; "An UTF-8 string" => String);

impl_config_kind!(TypeKind::String; "String"; "A socket address" => std::net::SocketAddr);
impl_config_kind!(TypeKind::String; "String"; "An IPv4 socket address" => std::net::SocketAddrV4);
impl_config_kind!(TypeKind::String; "String"; "An IPv6 socket address" => std::net::SocketAddrV6);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{AsTypeDescription, TypeDescription, TypeKind};

    #[test]
    fn verify_correct_config_kinds() {
        assert!(matches!(
            Vec::<f64>::as_type_description(),
            TypeDescription {
                doc: None,
                kind: TypeKind::Array(x),
                ..
            } if matches!(x.kind(), TypeKind::Float)
        ));

        let complex_config = HashMap::<String, Vec<HashMap<String, String>>>::as_type_description();
        println!("Complex config: {:#?}", complex_config);

        assert!(
            matches!(complex_config.kind(), TypeKind::HashMap(map) if matches!(map.kind(), TypeKind::Array(arr) if matches!(arr.kind(), TypeKind::HashMap(inner_map) if matches!(inner_map.kind(), TypeKind::String))))
        );
    }
}
