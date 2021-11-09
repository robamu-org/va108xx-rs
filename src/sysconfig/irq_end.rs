#[doc = "Register `IRQ_END` reader"]
pub struct R(crate::R<IRQ_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_END_SPEC>) -> Self {
        R(reader)
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
impl R {
    #[doc = "Bit 0 - RAM Single Bit Interrupt"]
    #[inline(always)]
    pub fn ramsbe(&self) -> RAMSBE_R {
        RAMSBE_R::new((self.bits & 0x01) != 0)
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
#[doc = "Enabled EDAC Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_end](index.html) module"]
pub struct IRQ_END_SPEC;
impl crate::RegisterSpec for IRQ_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_end::R](R) reader structure"]
impl crate::Readable for IRQ_END_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_END to value 0"]
impl crate::Resettable for IRQ_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
