#[doc = "Register `LOCKUP_RESET` reader"]
pub struct R(crate::R<LOCKUP_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKUP_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKUP_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKUP_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKUP_RESET` writer"]
pub struct W(crate::W<LOCKUP_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKUP_RESET_SPEC>;
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
impl From<crate::W<LOCKUP_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKUP_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LREN` reader - Lockup Reset Enable Bit"]
pub struct LREN_R(crate::FieldReader<bool, bool>);
impl LREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LREN` writer - Lockup Reset Enable Bit"]
pub struct LREN_W<'a> {
    w: &'a mut W,
}
impl<'a> LREN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Lockup Reset Enable Bit"]
    #[inline(always)]
    pub fn lren(&self) -> LREN_R {
        LREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lockup Reset Enable Bit"]
    #[inline(always)]
    pub fn lren(&mut self) -> LREN_W {
        LREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lockup Reset Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockup_reset](index.html) module"]
pub struct LOCKUP_RESET_SPEC;
impl crate::RegisterSpec for LOCKUP_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockup_reset::R](R) reader structure"]
impl crate::Readable for LOCKUP_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockup_reset::W](W) writer structure"]
impl crate::Writable for LOCKUP_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCKUP_RESET to value 0x01"]
impl crate::Resettable for LOCKUP_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
