// @generated
/// Implement `DataProvider<BidiClassNameToValueV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_from_bc_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_FROM_BC_V1: &'static <icu_properties::provider::BidiClassNameToValueV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::names::PropertyValueNameToEnumMapV1 {
                map: unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b".\0\0\0\0\0\x02\0\x04\0\x11\0\x1E\0\x1F\0!\x001\0A\0C\0E\0G\0I\0X\0j\0}\0\x91\0\x94\0\x95\0\xA2\0\xB9\0\xCE\0\xE4\0\xE7\0\xEA\0\xED\0\xFC\0\xFF\0\x01\x01\x0E\x01!\x01$\x01'\x01=\x01T\x01U\x01b\x01y\x01\x8E\x01\xA4\x01\xA7\x01\xAA\x01\xAD\x01\xAE\x01\xBF\x01\xCA\x01ALANArabic_LetterArabic_NumberBBNBoundary_NeutralCommon_SeparatorCSENESETEuropean_NumberEuropean_SeparatorEuropean_TerminatorFirst_Strong_IsolateFSILLeft_To_RightLeft_To_Right_EmbeddingLeft_To_Right_IsolateLeft_To_Right_OverrideLRELRILRONonspacing_MarkNSMONOther_NeutralParagraph_SeparatorPDFPDIPop_Directional_FormatPop_Directional_IsolateRRight_To_LeftRight_To_Left_EmbeddingRight_To_Left_IsolateRight_To_Left_OverrideRLERLIRLOSSegment_SeparatorWhite_SpaceWS") }, unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\r\0\x05\0\r\0\x05\0\x07\0\x12\0\x12\0\x06\0\x06\0\x02\0\x03\0\x04\0\x02\0\x03\0\x04\0\x13\0\x13\0\0\0\0\0\x0B\0\x14\0\x0C\0\x0B\0\x14\0\x0C\0\x11\0\x11\0\n\0\n\0\x07\0\x10\0\x16\0\x10\0\x16\0\x01\0\x01\0\x0E\0\x15\0\x0F\0\x0E\0\x15\0\x0F\0\x08\0\x08\0\t\0\t\0") })
                },
            };
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu_properties::provider::BidiClassNameToValueV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::BidiClassNameToValueV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_FROM_BC_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu_properties::provider::BidiClassNameToValueV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
