#[doc = "Register `RAM_MBE` reader"]
pub struct R(crate::R<RAM_MBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_MBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_MBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_MBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_MBE` writer"]
pub struct W(crate::W<RAM_MBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_MBE_SPEC>;
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
impl From<crate::W<RAM_MBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_MBE_SPEC>) -> Self {
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
#[doc = "Count of RAM EDAC Multi Bit Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_mbe](index.html) module"]
pub struct RAM_MBE_SPEC;
impl crate::RegisterSpec for RAM_MBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_mbe::R](R) reader structure"]
impl crate::Readable for RAM_MBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_mbe::W](W) writer structure"]
impl crate::Writable for RAM_MBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_MBE to value 0"]
impl crate::Resettable for RAM_MBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
