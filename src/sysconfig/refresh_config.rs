#[doc = "Register `REFRESH_CONFIG` reader"]
pub struct R(crate::R<REFRESH_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFRESH_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFRESH_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFRESH_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFRESH_CONFIG` writer"]
pub struct W(crate::W<REFRESH_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFRESH_CONFIG_SPEC>;
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
impl From<crate::W<REFRESH_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFRESH_CONFIG_SPEC>) -> Self {
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
#[doc = "Register Refresh Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refresh_config](index.html) module"]
pub struct REFRESH_CONFIG_SPEC;
impl crate::RegisterSpec for REFRESH_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refresh_config::R](R) reader structure"]
impl crate::Readable for REFRESH_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refresh_config::W](W) writer structure"]
impl crate::Writable for REFRESH_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFRESH_CONFIG to value 0"]
impl crate::Resettable for REFRESH_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
