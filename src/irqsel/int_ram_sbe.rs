#[doc = "Register `INT_RAM_SBE` reader"]
pub struct R(crate::R<INT_RAM_SBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAM_SBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAM_SBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAM_SBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAM_SBE` writer"]
pub struct W(crate::W<INT_RAM_SBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAM_SBE_SPEC>;
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
impl From<crate::W<INT_RAM_SBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAM_SBE_SPEC>) -> Self {
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
#[doc = "Internal Memory RAM SBE Interrupt Redirect Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ram_sbe](index.html) module"]
pub struct INT_RAM_SBE_SPEC;
impl crate::RegisterSpec for INT_RAM_SBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ram_sbe::R](R) reader structure"]
impl crate::Readable for INT_RAM_SBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ram_sbe::W](W) writer structure"]
impl crate::Writable for INT_RAM_SBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAM_SBE to value 0xffff_ffff"]
impl crate::Resettable for INT_RAM_SBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
