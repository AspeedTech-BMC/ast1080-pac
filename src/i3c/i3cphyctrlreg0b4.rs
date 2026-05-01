#[doc = "Register `I3CPHYCTRLREG0B4` reader"]
pub type R = crate::R<I3cphyctrlreg0b4Spec>;
#[doc = "Register `I3CPHYCTRLREG0B4` writer"]
pub type W = crate::W<I3cphyctrlreg0b4Spec>;
#[doc = "Field `REGCNTBUSFREEFMP` reader - REG_CNT_BUS_FREE_FMP"]
pub type RegcntbusfreefmpR = crate::FieldReader;
#[doc = "Field `REGCNTBUSFREEFMP` writer - REG_CNT_BUS_FREE_FMP"]
pub type RegcntbusfreefmpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGCNTBUSFREEPURE` reader - REG_CNT_BUS_FREE_PURE"]
pub type RegcntbusfreepureR = crate::FieldReader;
#[doc = "Field `REGCNTBUSFREEPURE` writer - REG_CNT_BUS_FREE_PURE"]
pub type RegcntbusfreepureW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGCNT01US` reader - REG_CNT_01US"]
pub type Regcnt01usR = crate::FieldReader;
#[doc = "Field `REGCNT01US` writer - REG_CNT_01US"]
pub type Regcnt01usW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_CNT_BUS_FREE_FMP"]
    #[inline(always)]
    pub fn regcntbusfreefmp(&self) -> RegcntbusfreefmpR {
        RegcntbusfreefmpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_CNT_BUS_FREE_PURE"]
    #[inline(always)]
    pub fn regcntbusfreepure(&self) -> RegcntbusfreepureR {
        RegcntbusfreepureR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_CNT_01US"]
    #[inline(always)]
    pub fn regcnt01us(&self) -> Regcnt01usR {
        Regcnt01usR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_CNT_BUS_FREE_FMP"]
    #[inline(always)]
    pub fn regcntbusfreefmp(&mut self) -> RegcntbusfreefmpW<I3cphyctrlreg0b4Spec> {
        RegcntbusfreefmpW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_CNT_BUS_FREE_PURE"]
    #[inline(always)]
    pub fn regcntbusfreepure(&mut self) -> RegcntbusfreepureW<I3cphyctrlreg0b4Spec> {
        RegcntbusfreepureW::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_CNT_01US"]
    #[inline(always)]
    pub fn regcnt01us(&mut self) -> Regcnt01usW<I3cphyctrlreg0b4Spec> {
        Regcnt01usW::new(self, 16)
    }
}
#[doc = "SDA\\_DETECTOR\\_CNT0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0b4Spec;
impl crate::RegisterSpec for I3cphyctrlreg0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0b4::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0b4::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0B4 to value 0x0013_0704"]
impl crate::Resettable for I3cphyctrlreg0b4Spec {
    const RESET_VALUE: u32 = 0x0013_0704;
}
