// @generated
/// Implement `DataProvider<GeneralCategoryNameToValueV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_gc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_GC_V1: &'static <icu_properties::provider::GeneralCategoryNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b">\0\0\0\0\0\x02\0\x04\0\x15\0\x17\0\x1C\0\x1E\x003\0:\0<\0K\0[\0i\0n\0|\0\x8D\0\x93\0\xA6\0\xB3\0\xC1\0\xC3\0\xC5\0\xC7\0\xD7\0\xD9\0\xDB\0\xE6\0\xE8\0\xEA\0\xEC\0\xFB\0\n\x01\x0C\x01\x0E\x01\x10\x01\x1F\x01/\x01;\x01G\x01X\x01d\x01w\x01y\x01{\x01}\x01\x7F\x01\x81\x01\x83\x01\x8E\x01\x90\x01\x92\x01\x94\x01\x96\x01\x98\x01\xA7\x01\xB3\x01\xBC\x01\xCC\x01\xD6\x01\xE6\x01\xE8\x01\xEA\x01CcCfClose_PunctuationCncntrlCoConnector_PunctuationControlCsCurrency_SymbolDash_PunctuationDecimal_NumberdigitEnclosing_MarkFinal_PunctuationFormatInitial_PunctuationLetter_NumberLine_SeparatorLlLmLoLowercase_LetterLtLuMath_SymbolMcMeMnModifier_LetterModifier_SymbolNdNlNoNonspacing_MarkOpen_PunctuationOther_LetterOther_NumberOther_PunctuationOther_SymbolParagraph_SeparatorPcPdPePfPiPoPrivate_UsePsScSkSmSoSpace_SeparatorSpacing_MarkSurrogateTitlecase_LetterUnassignedUppercase_LetterZlZpZs") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x0F\0\x10\0\x15\0\0\0\x0F\0\x11\0\x16\0\x0F\0\x12\0\x19\0\x13\0\t\0\t\0\x07\0\x1D\0\x10\0\x1C\0\n\0\r\0\x02\0\x04\0\x05\0\x02\0\x03\0\x01\0\x18\0\x08\0\x07\0\x06\0\x04\0\x1A\0\t\0\n\0\x0B\0\x06\0\x14\0\x05\0\x0B\0\x17\0\x1B\0\x0E\0\x16\0\x13\0\x15\0\x1D\0\x1C\0\x17\0\x11\0\x14\0\x19\0\x1A\0\x18\0\x1B\0\x0C\0\x08\0\x12\0\x03\0\0\0\x01\0\r\0\x0E\0\x0C\0") })
                },
            };
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::GeneralCategoryNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::GeneralCategoryNameToValueV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_FROM_GC_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::GeneralCategoryNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
