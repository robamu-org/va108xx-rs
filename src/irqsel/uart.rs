#[doc = "Register `UART[%s]` reader"]
pub struct R(crate::R<UART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART[%s]` writer"]
pub struct W(crate::W<UART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SPEC>;
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
impl From<crate::W<UART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SPEC>) -> Self {
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
#[doc = "UART Interrupt Redirect Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart](index.html) module"]
pub struct UART_SPEC;
impl crate::RegisterSpec for UART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart::R](R) reader structure"]
impl crate::Readable for UART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart::W](W) writer structure"]
impl crate::Writable for UART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART[%s]
to value 0xffff_ffff"]
impl crate::Resettable for UART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
