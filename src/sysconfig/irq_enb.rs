#[doc = "Register `IRQ_ENB` reader"]
pub struct R(crate::R<IRQ_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_ENB` writer"]
pub struct W(crate::W<IRQ_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENB_SPEC>;
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
impl From<crate::W<IRQ_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMSBE` reader - RAM Single Bit Interrupt"]
pub struct RAMSBE_R(crate::FieldReader<bool, bool>);
impl RAMSBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMSBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMSBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMSBE` writer - RAM Single Bit Interrupt"]
pub struct RAMSBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMSBE_W<'a> {
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
        self.w.bits = value as u32;
        self.w
    }
}
#[doc = "Field `RAMMBE` reader - RAM Multi Bit Interrupt"]
pub struct RAMMBE_R(crate::FieldReader<bool, bool>);
impl RAMMBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMMBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMMBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMMBE` writer - RAM Multi Bit Interrupt"]
pub struct RAMMBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMMBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ROMSBE` reader - ROM Single Bit Interrupt"]
pub struct ROMSBE_R(crate::FieldReader<bool, bool>);
impl ROMSBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMSBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMSBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROMSBE` writer - ROM Single Bit Interrupt"]
pub struct ROMSBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMSBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ROMMBE` reader - ROM Multi Bit Interrupt"]
pub struct ROMMBE_R(crate::FieldReader<bool, bool>);
impl ROMMBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMMBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMMBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROMMBE` writer - ROM Multi Bit Interrupt"]
pub struct ROMMBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMMBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RAM Single Bit Interrupt"]
    #[inline(always)]
    pub fn ramsbe(&self) -> RAMSBE_R {
        RAMSBE_R::new(self.bits != 0)
    }
    #[doc = "Bit 1 - RAM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rammbe(&self) -> RAMMBE_R {
        RAMMBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&self) -> ROMSBE_R {
        ROMSBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&self) -> ROMMBE_R {
        ROMMBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM Single Bit Interrupt"]
    #[inline(always)]
    pub fn ramsbe(&mut self) -> RAMSBE_W {
        RAMSBE_W { w: self }
    }
    #[doc = "Bit 1 - RAM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rammbe(&mut self) -> RAMMBE_W {
        RAMMBE_W { w: self }
    }
    #[doc = "Bit 2 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&mut self) -> ROMSBE_W {
        ROMSBE_W { w: self }
    }
    #[doc = "Bit 3 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&mut self) -> ROMMBE_W {
        ROMMBE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable EDAC Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enb](index.html) module"]
pub struct IRQ_ENB_SPEC;
impl crate::RegisterSpec for IRQ_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_enb::R](R) reader structure"]
impl crate::Readable for IRQ_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_enb::W](W) writer structure"]
impl crate::Writable for IRQ_ENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IRQ_ENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
