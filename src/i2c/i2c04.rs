#[doc = "Register `I2C04` reader"]
pub type R = crate::R<I2c04Spec>;
#[doc = "Register `I2C04` writer"]
pub type W = crate::W<I2c04Spec>;
#[doc = "Field `ACTIMEBASE` reader - ACTIME_BASE"]
pub type ActimebaseR = crate::FieldReader;
#[doc = "Field `ACTIMEBASE` writer - ACTIME_BASE"]
pub type ActimebaseW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMEOUTBASE` reader - TIMEOUT_BASE"]
pub type TimeoutbaseR = crate::FieldReader;
#[doc = "Field `TIMEOUTBASE` writer - TIMEOUT_BASE"]
pub type TimeoutbaseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `THDDAT` reader - THDDAT"]
pub type ThddatR = crate::FieldReader;
#[doc = "Field `THDDAT` writer - THDDAT"]
pub type ThddatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCKLOW` reader - TCKLOW"]
pub type TcklowR = crate::FieldReader;
#[doc = "Field `TCKLOW` writer - TCKLOW"]
pub type TcklowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKHIGH` reader - TCKHIGH"]
pub type TckhighR = crate::FieldReader;
#[doc = "Field `TCKHIGH` writer - TCKHIGH"]
pub type TckhighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKHIGHMIN` reader - TCKHIGHMIN"]
pub type TckhighminR = crate::FieldReader;
#[doc = "Field `TCKHIGHMIN` writer - TCKHIGHMIN"]
pub type TckhighminW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TTIMEOUT` reader - TTIMEOUT"]
pub type TtimeoutR = crate::FieldReader;
#[doc = "Field `TTIMEOUT` writer - TTIMEOUT"]
pub type TtimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - ACTIME_BASE"]
    #[inline(always)]
    pub fn actimebase(&self) -> ActimebaseR {
        ActimebaseR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - TIMEOUT_BASE"]
    #[inline(always)]
    pub fn timeoutbase(&self) -> TimeoutbaseR {
        TimeoutbaseR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - THDDAT"]
    #[inline(always)]
    pub fn thddat(&self) -> ThddatR {
        ThddatR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - TCKLOW"]
    #[inline(always)]
    pub fn tcklow(&self) -> TcklowR {
        TcklowR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TCKHIGH"]
    #[inline(always)]
    pub fn tckhigh(&self) -> TckhighR {
        TckhighR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TCKHIGHMIN"]
    #[inline(always)]
    pub fn tckhighmin(&self) -> TckhighminR {
        TckhighminR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - TTIMEOUT"]
    #[inline(always)]
    pub fn ttimeout(&self) -> TtimeoutR {
        TtimeoutR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ACTIME_BASE"]
    #[inline(always)]
    pub fn actimebase(&mut self) -> ActimebaseW<I2c04Spec> {
        ActimebaseW::new(self, 0)
    }
    #[doc = "Bits 8:9 - TIMEOUT_BASE"]
    #[inline(always)]
    pub fn timeoutbase(&mut self) -> TimeoutbaseW<I2c04Spec> {
        TimeoutbaseW::new(self, 8)
    }
    #[doc = "Bits 10:11 - THDDAT"]
    #[inline(always)]
    pub fn thddat(&mut self) -> ThddatW<I2c04Spec> {
        ThddatW::new(self, 10)
    }
    #[doc = "Bits 12:15 - TCKLOW"]
    #[inline(always)]
    pub fn tcklow(&mut self) -> TcklowW<I2c04Spec> {
        TcklowW::new(self, 12)
    }
    #[doc = "Bits 16:19 - TCKHIGH"]
    #[inline(always)]
    pub fn tckhigh(&mut self) -> TckhighW<I2c04Spec> {
        TckhighW::new(self, 16)
    }
    #[doc = "Bits 20:23 - TCKHIGHMIN"]
    #[inline(always)]
    pub fn tckhighmin(&mut self) -> TckhighminW<I2c04Spec> {
        TckhighminW::new(self, 20)
    }
    #[doc = "Bits 24:29 - TTIMEOUT"]
    #[inline(always)]
    pub fn ttimeout(&mut self) -> TtimeoutW<I2c04Spec> {
        TtimeoutW::new(self, 24)
    }
}
#[doc = "Master/Slave Clock and AC Timing Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c04Spec;
impl crate::RegisterSpec for I2c04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c04::R`](R) reader structure"]
impl crate::Readable for I2c04Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c04::W`](W) writer structure"]
impl crate::Writable for I2c04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C04 to value 0x08"]
impl crate::Resettable for I2c04Spec {
    const RESET_VALUE: u32 = 0x08;
}
