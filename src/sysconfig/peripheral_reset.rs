#[doc = "Register `PERIPHERAL_RESET` reader"]
pub struct R(crate::R<PERIPHERAL_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPHERAL_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPHERAL_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPHERAL_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPHERAL_RESET` writer"]
pub struct W(crate::W<PERIPHERAL_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPHERAL_RESET_SPEC>;
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
impl From<crate::W<PERIPHERAL_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPHERAL_RESET_SPEC>) -> Self {
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
#[doc = "Peripheral Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peripheral_reset](index.html) module"]
pub struct PERIPHERAL_RESET_SPEC;
impl crate::RegisterSpec for PERIPHERAL_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peripheral_reset::R](R) reader structure"]
impl crate::Readable for PERIPHERAL_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peripheral_reset::W](W) writer structure"]
impl crate::Writable for PERIPHERAL_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPHERAL_RESET to value 0xffff_ffff"]
impl crate::Resettable for PERIPHERAL_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
