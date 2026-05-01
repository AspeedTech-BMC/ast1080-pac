#[doc = "Register `I3CPHYCTRLREG0DC` reader"]
pub type R = crate::R<I3cphyctrlreg0dcSpec>;
#[doc = "Register `I3CPHYCTRLREG0DC` writer"]
pub type W = crate::W<I3cphyctrlreg0dcSpec>;
#[doc = "Field `REGCRSTOPBUSFREECNT` reader - REG_CR_STOP_BUS_FREE_CNT"]
pub type RegcrstopbusfreecntR = crate::FieldReader<u16>;
#[doc = "Field `REGCRSTOPBUSFREECNT` writer - REG_CR_STOP_BUS_FREE_CNT"]
pub type RegcrstopbusfreecntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_CR_STOP_BUS_FREE_CNT"]
    #[inline(always)]
    pub fn regcrstopbusfreecnt(&self) -> RegcrstopbusfreecntR {
        RegcrstopbusfreecntR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_CR_STOP_BUS_FREE_CNT"]
    #[inline(always)]
    pub fn regcrstopbusfreecnt(&mut self) -> RegcrstopbusfreecntW<I3cphyctrlreg0dcSpec> {
        RegcrstopbusfreecntW::new(self, 0)
    }
}
#[doc = "BUS\\_FREE\\_TIME\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0dcSpec;
impl crate::RegisterSpec for I3cphyctrlreg0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0dc::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0dc::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0DC to value 0x06"]
impl crate::Resettable for I3cphyctrlreg0dcSpec {
    const RESET_VALUE: u32 = 0x06;
}
