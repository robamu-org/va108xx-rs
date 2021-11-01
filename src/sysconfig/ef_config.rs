#[doc = "Register `EF_CONFIG` reader"]
pub struct R(crate::R<EF_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EFuse Config Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_config](index.html) module"]
pub struct EF_CONFIG_SPEC;
impl crate::RegisterSpec for EF_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_config::R](R) reader structure"]
impl crate::Readable for EF_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EF_CONFIG to value 0"]
impl crate::Resettable for EF_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
