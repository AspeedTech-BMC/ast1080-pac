#[doc = "Register `I3CPHYCTRLREG0B8` reader"]
pub type R = crate::R<I3cphyctrlreg0b8Spec>;
#[doc = "Register `I3CPHYCTRLREG0B8` writer"]
pub type W = crate::W<I3cphyctrlreg0b8Spec>;
#[doc = "Field `REGCNTBUSIDLE` reader - REG_CNT_BUS_IDLE"]
pub type RegcntbusidleR = crate::FieldReader<u16>;
#[doc = "Field `REGCNTBUSIDLE` writer - REG_CNT_BUS_IDLE"]
pub type RegcntbusidleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGCNTBUSFREEFM` reader - REG_CNT_BUS_FREE_FM"]
pub type RegcntbusfreefmR = crate::FieldReader;
#[doc = "Field `REGCNTBUSFREEFM` writer - REG_CNT_BUS_FREE_FM"]
pub type RegcntbusfreefmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGCNTBUSAVAL` reader - REG_CNT_BUS_AVAL"]
pub type RegcntbusavalR = crate::FieldReader;
#[doc = "Field `REGCNTBUSAVAL` writer - REG_CNT_BUS_AVAL"]
pub type RegcntbusavalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - REG_CNT_BUS_IDLE"]
    #[inline(always)]
    pub fn regcntbusidle(&self) -> RegcntbusidleR {
        RegcntbusidleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - REG_CNT_BUS_FREE_FM"]
    #[inline(always)]
    pub fn regcntbusfreefm(&self) -> RegcntbusfreefmR {
        RegcntbusfreefmR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_CNT_BUS_AVAL"]
    #[inline(always)]
    pub fn regcntbusaval(&self) -> RegcntbusavalR {
        RegcntbusavalR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_CNT_BUS_IDLE"]
    #[inline(always)]
    pub fn regcntbusidle(&mut self) -> RegcntbusidleW<I3cphyctrlreg0b8Spec> {
        RegcntbusidleW::new(self, 0)
    }
    #[doc = "Bits 16:23 - REG_CNT_BUS_FREE_FM"]
    #[inline(always)]
    pub fn regcntbusfreefm(&mut self) -> RegcntbusfreefmW<I3cphyctrlreg0b8Spec> {
        RegcntbusfreefmW::new(self, 16)
    }
    #[doc = "Bits 24:31 - REG_CNT_BUS_AVAL"]
    #[inline(always)]
    pub fn regcntbusaval(&mut self) -> RegcntbusavalW<I3cphyctrlreg0b8Spec> {
        RegcntbusavalW::new(self, 24)
    }
}
#[doc = "SDA\\_DETECTOR\\_CNT1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0b8Spec;
impl crate::RegisterSpec for I3cphyctrlreg0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0b8::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0b8::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0B8 to value 0x090c_00c7"]
impl crate::Resettable for I3cphyctrlreg0b8Spec {
    const RESET_VALUE: u32 = 0x090c_00c7;
}
