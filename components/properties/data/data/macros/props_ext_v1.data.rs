// @generated
/// Implement `DataProvider<ExtenderV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ext_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_EXT_V1: &'static <icu_properties::provider::ExtenderV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xB7\0\0\0\xB8\0\0\0\xD0\x02\0\0\xD2\x02\0\0@\x06\0\0A\x06\0\0\xFA\x07\0\0\xFB\x07\0\0U\x0B\0\0V\x0B\0\0F\x0E\0\0G\x0E\0\0\xC6\x0E\0\0\xC7\x0E\0\0\n\x18\0\0\x0B\x18\0\0C\x18\0\0D\x18\0\0\xA7\x1A\0\0\xA8\x1A\0\x006\x1C\0\x007\x1C\0\0{\x1C\0\0|\x1C\0\0\x050\0\0\x060\0\x0010\0\x0060\0\0\x9D0\0\0\x9F0\0\0\xFC0\0\0\xFF0\0\0\x15\xA0\0\0\x16\xA0\0\0\x0C\xA6\0\0\r\xA6\0\0\xCF\xA9\0\0\xD0\xA9\0\0\xE6\xA9\0\0\xE7\xA9\0\0p\xAA\0\0q\xAA\0\0\xDD\xAA\0\0\xDE\xAA\0\0\xF3\xAA\0\0\xF5\xAA\0\0p\xFF\0\0q\xFF\0\0\x81\x07\x01\0\x83\x07\x01\0]\x13\x01\0^\x13\x01\0\xC6\x15\x01\0\xC9\x15\x01\0\x98\x1A\x01\0\x99\x1A\x01\0Bk\x01\0Dk\x01\0\xE0o\x01\0\xE2o\x01\0\xE3o\x01\0\xE4o\x01\0<\xE1\x01\0>\xE1\x01\0D\xE9\x01\0G\xE9\x01\0") }, 50u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::ExtenderV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::ExtenderV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_EXT_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::ExtenderV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
