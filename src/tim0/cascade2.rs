#[doc = "Register `CASCADE2` reader"]
pub struct R(crate::R<CASCADE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CASCADE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CASCADE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CASCADE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CASCADE2` writer"]
pub struct W(crate::W<CASCADE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CASCADE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CASCADE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CASCADE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CASSEL` reader - Cascade Selection"]
pub struct CASSEL_R(crate::FieldReader<u8, u8>);
impl CASSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CASSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CASSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CASSEL` writer - Cascade Selection"]
pub struct CASSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CASSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Cascade Selection"]
    #[inline(always)]
    pub fn cassel(&self) -> CASSEL_R {
        CASSEL_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cascade Selection"]
    #[inline(always)]
    pub fn cassel(&mut self) -> CASSEL_W {
        CASSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cascade Enable Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cascade2](index.html) module"]
pub struct CASCADE2_SPEC;
impl crate::RegisterSpec for CASCADE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cascade2::R](R) reader structure"]
impl crate::Readable for CASCADE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cascade2::W](W) writer structure"]
impl crate::Writable for CASCADE2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CASCADE2 to value 0"]
impl crate::Resettable for CASCADE2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
