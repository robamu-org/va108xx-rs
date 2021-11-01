#[doc = "Register `TXEV` reader"]
pub struct R(crate::R<TXEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXEV` writer"]
pub struct W(crate::W<TXEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEV_SPEC>;
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
impl From<crate::W<TXEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor TXEV Interrupt Redirect Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txev](index.html) module"]
pub struct TXEV_SPEC;
impl crate::RegisterSpec for TXEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txev::R](R) reader structure"]
impl crate::Readable for TXEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txev::W](W) writer structure"]
impl crate::Writable for TXEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXEV to value 0xffff_ffff"]
impl crate::Resettable for TXEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
