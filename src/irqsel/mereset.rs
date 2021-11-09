#[doc = "Register `MERESET` reader"]
pub struct R(crate::R<MERESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MERESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MERESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MERESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE` reader - Active"]
pub struct ACTIVE_R(crate::FieldReader<bool, bool>);
impl ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "MERESET Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mereset](index.html) module"]
pub struct MERESET_SPEC;
impl crate::RegisterSpec for MERESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mereset::R](R) reader structure"]
impl crate::Readable for MERESET_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MERESET to value 0"]
impl crate::Resettable for MERESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
