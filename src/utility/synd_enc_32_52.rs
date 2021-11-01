#[doc = "Register `SYND_ENC_32_52` reader"]
pub struct R(crate::R<SYND_ENC_32_52_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYND_ENC_32_52_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYND_ENC_32_52_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYND_ENC_32_52_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Synd 32/52 bit Encoded Syndrome\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synd_enc_32_52](index.html) module"]
pub struct SYND_ENC_32_52_SPEC;
impl crate::RegisterSpec for SYND_ENC_32_52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synd_enc_32_52::R](R) reader structure"]
impl crate::Readable for SYND_ENC_32_52_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYND_ENC_32_52 to value 0"]
impl crate::Resettable for SYND_ENC_32_52_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
