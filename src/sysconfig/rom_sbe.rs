#[doc = "Register `ROM_SBE` reader"]
pub struct R(crate::R<ROM_SBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_SBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_SBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_SBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_SBE` writer"]
pub struct W(crate::W<ROM_SBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_SBE_SPEC>;
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
impl From<crate::W<ROM_SBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_SBE_SPEC>) -> Self {
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
#[doc = "Count of ROM EDAC Single Bit Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_sbe](index.html) module"]
pub struct ROM_SBE_SPEC;
impl crate::RegisterSpec for ROM_SBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_sbe::R](R) reader structure"]
impl crate::Readable for ROM_SBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_sbe::W](W) writer structure"]
impl crate::Writable for ROM_SBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_SBE to value 0"]
impl crate::Resettable for ROM_SBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
