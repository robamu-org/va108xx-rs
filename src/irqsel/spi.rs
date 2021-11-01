#[doc = "Register `SPI[%s]` reader"]
pub struct R(crate::R<SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI[%s]` writer"]
pub struct W(crate::W<SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SPEC>;
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
impl From<crate::W<SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SPEC>) -> Self {
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
#[doc = "SPI Interrupt Redirect Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi](index.html) module"]
pub struct SPI_SPEC;
impl crate::RegisterSpec for SPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi::R](R) reader structure"]
impl crate::Readable for SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi::W](W) writer structure"]
impl crate::Writable for SPI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI[%s]
to value 0xffff_ffff"]
impl crate::Resettable for SPI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
