// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/color_model.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(non_camel_case_types)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: Specified what color components are present in an [`archetypes::Image`][crate::archetypes::Image].
///
/// This combined with [`datatypes::ChannelDatatype`][crate::datatypes::ChannelDatatype] determines the pixel format of an image.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ColorModel {
    /// Grayscale luminance intencity/brightness/value, sometimes called `Y`
    #[default]
    L = 1,

    /// Red, Green, Blue
    #[allow(clippy::upper_case_acronyms)]
    RGB = 2,

    /// Red, Green, Blue, Alpha
    #[allow(clippy::upper_case_acronyms)]
    RGBA = 3,

    /// Blue, Green, Red
    #[allow(clippy::upper_case_acronyms)]
    BGR = 4,

    /// Blue, Green, Red, Alpha
    #[allow(clippy::upper_case_acronyms)]
    BGRA = 5,
}

impl ::re_types_core::reflection::Enum for ColorModel {
    #[inline]
    fn variants() -> &'static [Self] {
        &[Self::L, Self::RGB, Self::RGBA, Self::BGR, Self::BGRA]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::L => "Grayscale luminance intencity/brightness/value, sometimes called `Y`",
            Self::RGB => "Red, Green, Blue",
            Self::RGBA => "Red, Green, Blue, Alpha",
            Self::BGR => "Blue, Green, Red",
            Self::BGRA => "Blue, Green, Red, Alpha",
        }
    }
}

impl ::re_types_core::SizeBytes for ColorModel {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}

impl std::fmt::Display for ColorModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::L => write!(f, "L"),
            Self::RGB => write!(f, "RGB"),
            Self::RGBA => write!(f, "RGBA"),
            Self::BGR => write!(f, "BGR"),
            Self::BGRA => write!(f, "BGRA"),
        }
    }
}

::re_types_core::macros::impl_into_cow!(ColorModel);

impl ::re_types_core::Loggable for ColorModel {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::UInt8
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};

        #[allow(unused)]
        fn as_array_ref<T: Array + 'static>(t: T) -> ArrayRef {
            std::sync::Arc::new(t) as ArrayRef
        }
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| *datum as u8);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(PrimitiveArray::<UInt8Type>::new(
                ScalarBuffer::from(
                    data0
                        .into_iter()
                        .map(|v| v.unwrap_or_default())
                        .collect::<Vec<_>>(),
                ),
                data0_validity,
            ))
        })
    }

    fn from_arrow2_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow::datatypes::*;
        use arrow2::{array::*, buffer::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<UInt8Array>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.datatypes.ColorModel#enum")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|typ| match typ {
                Some(1) => Ok(Some(Self::L)),
                Some(2) => Ok(Some(Self::RGB)),
                Some(3) => Ok(Some(Self::RGBA)),
                Some(4) => Ok(Some(Self::BGR)),
                Some(5) => Ok(Some(Self::BGRA)),
                None => Ok(None),
                Some(invalid) => Err(DeserializationError::missing_union_arm(
                    Self::arrow_datatype(),
                    "<invalid>",
                    invalid as _,
                )),
            })
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.datatypes.ColorModel")?)
    }
}
