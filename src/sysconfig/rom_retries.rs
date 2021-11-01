#[doc = "Register `ROM_RETRIES` reader"]
pub struct R(crate::R<ROM_RETRIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_RETRIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_RETRIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_RETRIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ROM BOOT Retry count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_retries](index.html) module"]
pub struct ROM_RETRIES_SPEC;
impl crate::RegisterSpec for ROM_RETRIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_retries::R](R) reader structure"]
impl crate::Readable for ROM_RETRIES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ROM_RETRIES to value 0"]
impl crate::Resettable for ROM_RETRIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
