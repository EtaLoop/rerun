// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/testing/components/fuzzy.fbs".

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

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AffixFuzzer22(pub Option<crate::testing::datatypes::AffixFuzzer22>);

impl ::re_types_core::SizeBytes for AffixFuzzer22 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::testing::datatypes::AffixFuzzer22>>::is_pod()
    }
}

impl<T: Into<Option<crate::testing::datatypes::AffixFuzzer22>>> From<T> for AffixFuzzer22 {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<Option<crate::testing::datatypes::AffixFuzzer22>> for AffixFuzzer22 {
    #[inline]
    fn borrow(&self) -> &Option<crate::testing::datatypes::AffixFuzzer22> {
        &self.0
    }
}

impl std::ops::Deref for AffixFuzzer22 {
    type Target = Option<crate::testing::datatypes::AffixFuzzer22>;

    #[inline]
    fn deref(&self) -> &Option<crate::testing::datatypes::AffixFuzzer22> {
        &self.0
    }
}

impl std::ops::DerefMut for AffixFuzzer22 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Option<crate::testing::datatypes::AffixFuzzer22> {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer22);

impl ::re_types_core::Loggable for AffixFuzzer22 {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Struct(Fields::from(vec![Field::new(
            "fixed_sized_native",
            DataType::FixedSizeList(
                std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                4,
            ),
            false,
        )]))
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
                    let datum = datum.map(|datum| datum.into_owned().0).flatten();
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                _ = data0_validity;
                crate::testing::datatypes::AffixFuzzer22::to_arrow_opt(data0)?
            }
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
        Ok(
            crate::testing::datatypes::AffixFuzzer22::from_arrow2_opt(arrow_data)
                .with_context("rerun.testing.components.AffixFuzzer22#nullable_nested_array")?
                .into_iter()
                .map(Ok)
                .map(|res| res.map(|v| Some(Self(v))))
                .collect::<DeserializationResult<Vec<Option<_>>>>()
                .with_context("rerun.testing.components.AffixFuzzer22#nullable_nested_array")
                .with_context("rerun.testing.components.AffixFuzzer22")?,
        )
    }
}

impl ::re_types_core::Component for AffixFuzzer22 {
    #[inline]
    fn name() -> ComponentName {
        "rerun.testing.components.AffixFuzzer22".into()
    }
}
