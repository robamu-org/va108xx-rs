#[doc = "Register `RAM_TRAP_SYND` reader"]
pub struct R(crate::R<RAM_TRAP_SYND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_TRAP_SYND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_TRAP_SYND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_TRAP_SYND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_TRAP_SYND` writer"]
pub struct W(crate::W<RAM_TRAP_SYND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_TRAP_SYND_SPEC>;
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
impl From<crate::W<RAM_TRAP_SYND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_TRAP_SYND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYND` reader - Trap Syndrom Bits"]
pub struct SYND_R(crate::FieldReader<u32, u32>);
impl SYND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SYND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYND` writer - Trap Syndrom Bits"]
pub struct SYND_W<'a> {
    w: &'a mut W,
}
impl<'a> SYND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Trap Syndrom Bits"]
    #[inline(always)]
    pub fn synd(&self) -> SYND_R {
        SYND_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Trap Syndrom Bits"]
    #[inline(always)]
    pub fn synd(&mut self) -> SYND_W {
        SYND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Trap Syndrome\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_trap_synd](index.html) module"]
pub struct RAM_TRAP_SYND_SPEC;
impl crate::RegisterSpec for RAM_TRAP_SYND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_trap_synd::R](R) reader structure"]
impl crate::Readable for RAM_TRAP_SYND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_trap_synd::W](W) writer structure"]
impl crate::Writable for RAM_TRAP_SYND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_TRAP_SYND to value 0"]
impl crate::Resettable for RAM_TRAP_SYND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
