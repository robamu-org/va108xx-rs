#[doc = "Register `TIM_RESET` reader"]
pub struct R(crate::R<TIM_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM_RESET` writer"]
pub struct W(crate::W<TIM_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_RESET_SPEC>;
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
impl From<crate::W<TIM_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_RESET_SPEC>) -> Self {
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
#[doc = "TIM Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_reset](index.html) module"]
pub struct TIM_RESET_SPEC;
impl crate::RegisterSpec for TIM_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim_reset::R](R) reader structure"]
impl crate::Readable for TIM_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim_reset::W](W) writer structure"]
impl crate::Writable for TIM_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM_RESET to value 0xffff_ffff"]
impl crate::Resettable for TIM_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
