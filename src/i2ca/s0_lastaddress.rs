#[doc = "Register `S0_LASTADDRESS` reader"]
pub struct R(crate::R<S0_LASTADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_LASTADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_LASTADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_LASTADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Slave I2C Last Address value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_lastaddress](index.html) module"]
pub struct S0_LASTADDRESS_SPEC;
impl crate::RegisterSpec for S0_LASTADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_lastaddress::R](R) reader structure"]
impl crate::Readable for S0_LASTADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S0_LASTADDRESS to value 0"]
impl crate::Resettable for S0_LASTADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
