#[doc = "Register `PORTA[%s]` reader"]
pub struct R(crate::R<PORTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTA[%s]` writer"]
pub struct W(crate::W<PORTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTA_SPEC>;
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
impl From<crate::W<PORTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTA_SPEC>) -> Self {
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
#[doc = "PORTA Interrupt Redirect Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porta](index.html) module"]
pub struct PORTA_SPEC;
impl crate::RegisterSpec for PORTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [porta::R](R) reader structure"]
impl crate::Readable for PORTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [porta::W](W) writer structure"]
impl crate::Writable for PORTA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTA[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PORTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
