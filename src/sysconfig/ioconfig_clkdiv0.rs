#[doc = "Register `IOCONFIG_CLKDIV0` reader"]
pub struct R(crate::R<IOCONFIG_CLKDIV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCONFIG_CLKDIV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCONFIG_CLKDIV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCONFIG_CLKDIV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "IO Configuration Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconfig_clkdiv0](index.html) module"]
pub struct IOCONFIG_CLKDIV0_SPEC;
impl crate::RegisterSpec for IOCONFIG_CLKDIV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioconfig_clkdiv0::R](R) reader structure"]
impl crate::Readable for IOCONFIG_CLKDIV0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IOCONFIG_CLKDIV0 to value 0"]
impl crate::Resettable for IOCONFIG_CLKDIV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
