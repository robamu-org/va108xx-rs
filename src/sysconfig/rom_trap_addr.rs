#[doc = "Register `ROM_TRAP_ADDR` reader"]
pub struct R(crate::R<ROM_TRAP_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TRAP_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TRAP_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TRAP_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TRAP_ADDR` writer"]
pub struct W(crate::W<ROM_TRAP_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TRAP_ADDR_SPEC>;
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
impl From<crate::W<ROM_TRAP_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TRAP_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Trap Address Match Bits"]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Trap Address Match Bits"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Trap Enable Bit"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Trap Enable Bit"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Trap Address Match Bits"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Trap Enable Bit"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trap Address Match Bits"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 31 - Trap Enable Bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM Trap Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_trap_addr](index.html) module"]
pub struct ROM_TRAP_ADDR_SPEC;
impl crate::RegisterSpec for ROM_TRAP_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_trap_addr::R](R) reader structure"]
impl crate::Readable for ROM_TRAP_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_trap_addr::W](W) writer structure"]
impl crate::Writable for ROM_TRAP_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_TRAP_ADDR to value 0"]
impl crate::Resettable for ROM_TRAP_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
