#[doc = "Register `EF_ID` reader"]
pub struct R(crate::R<EF_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EFuse ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_id](index.html) module"]
pub struct EF_ID_SPEC;
impl crate::RegisterSpec for EF_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_id::R](R) reader structure"]
impl crate::Readable for EF_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EF_ID to value 0"]
impl crate::Resettable for EF_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
