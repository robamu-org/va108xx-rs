#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAITING` reader - Controller is Waiting"]
pub struct WAITING_R(crate::FieldReader<bool, bool>);
impl WAITING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLED` reader - Controller is Stalled"]
pub struct STALLED_R(crate::FieldReader<bool, bool>);
impl STALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLOST` reader - I2C Arbitration was lost"]
pub struct ARBLOST_R(crate::FieldReader<bool, bool>);
impl ARBLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKADDR` reader - I2C Address was not Acknowledged"]
pub struct NACKADDR_R(crate::FieldReader<bool, bool>);
impl NACKADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACKADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKDATA` reader - I2C Data was not Acknowledged"]
pub struct NACKDATA_R(crate::FieldReader<bool, bool>);
impl NACKDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACKDATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNEMPTY` reader - RX FIFO is Not Empty"]
pub struct RXNEMPTY_R(crate::FieldReader<bool, bool>);
impl RXNEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXNEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFULL` reader - RX FIFO is Full"]
pub struct RXFULL_R(crate::FieldReader<bool, bool>);
impl RXFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTRIGGER` reader - RX FIFO Above Trigger Level"]
pub struct RXTRIGGER_R(crate::FieldReader<bool, bool>);
impl RXTRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXTRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - TX FIFO is Empty"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXNFULL` reader - TX FIFO is Full"]
pub struct TXNFULL_R(crate::FieldReader<bool, bool>);
impl TXNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTRIGGER` reader - TX FIFO Below Trigger Level"]
pub struct TXTRIGGER_R(crate::FieldReader<bool, bool>);
impl TXTRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXTRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAW_SDA` reader - I2C Raw SDA value"]
pub struct RAW_SDA_R(crate::FieldReader<bool, bool>);
impl RAW_SDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAW_SDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAW_SDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAW_SCL` reader - I2C Raw SCL value"]
pub struct RAW_SCL_R(crate::FieldReader<bool, bool>);
impl RAW_SCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAW_SCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAW_SCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_IDLE` reader - I2C bus is Idle"]
pub struct I2C_IDLE_R(crate::FieldReader<bool, bool>);
impl I2C_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` reader - Controller is Idle"]
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controller is Stalled"]
    #[inline(always)]
    pub fn stalled(&self) -> STALLED_R {
        STALLED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration was lost"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Address was not Acknowledged"]
    #[inline(always)]
    pub fn nackaddr(&self) -> NACKADDR_R {
        NACKADDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&self) -> NACKDATA_R {
        NACKDATA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX FIFO is Not Empty"]
    #[inline(always)]
    pub fn rxnempty(&self) -> RXNEMPTY_R {
        RXNEMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RX FIFO is Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&self) -> RXTRIGGER_R {
        RXTRIGGER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TX FIFO is Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TX FIFO is Full"]
    #[inline(always)]
    pub fn txnfull(&self) -> TXNFULL_R {
        TXNFULL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&self) -> TXTRIGGER_R {
        TXTRIGGER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - I2C Raw SDA value"]
    #[inline(always)]
    pub fn raw_sda(&self) -> RAW_SDA_R {
        RAW_SDA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - I2C Raw SCL value"]
    #[inline(always)]
    pub fn raw_scl(&self) -> RAW_SCL_R {
        RAW_SCL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C bus is Idle"]
    #[inline(always)]
    pub fn i2c_idle(&self) -> I2C_IDLE_R {
        I2C_IDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controller is Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "I2C Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
