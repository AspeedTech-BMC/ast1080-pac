#[doc = "Register `I3CPHYCTRLREG08C` reader"]
pub type R = crate::R<I3cphyctrlreg08cSpec>;
#[doc = "Register `I3CPHYCTRLREG08C` writer"]
pub type W = crate::W<I3cphyctrlreg08cSpec>;
#[doc = "Field `REGSDRTGWRADDRACKPROLONGCNT` reader - REG_SDR_TG_WR_ADDR_ACK_PROLONG_CNT"]
pub type RegsdrtgwraddrackprolongcntR = crate::FieldReader<u16>;
#[doc = "Field `REGSDRTGWRADDRACKPROLONGCNT` writer - REG_SDR_TG_WR_ADDR_ACK_PROLONG_CNT"]
pub type RegsdrtgwraddrackprolongcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `REGSDRTGWRADDRACKPROLONGEN` reader - REG_SDR_TG_WR_ADDR_ACK_PROLONG_EN"]
pub type RegsdrtgwraddrackprolongenR = crate::BitReader;
#[doc = "Field `REGSDRTGWRADDRACKPROLONGEN` writer - REG_SDR_TG_WR_ADDR_ACK_PROLONG_EN"]
pub type RegsdrtgwraddrackprolongenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - REG_SDR_TG_WR_ADDR_ACK_PROLONG_CNT"]
    #[inline(always)]
    pub fn regsdrtgwraddrackprolongcnt(&self) -> RegsdrtgwraddrackprolongcntR {
        RegsdrtgwraddrackprolongcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - REG_SDR_TG_WR_ADDR_ACK_PROLONG_EN"]
    #[inline(always)]
    pub fn regsdrtgwraddrackprolongen(&self) -> RegsdrtgwraddrackprolongenR {
        RegsdrtgwraddrackprolongenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_SDR_TG_WR_ADDR_ACK_PROLONG_CNT"]
    #[inline(always)]
    pub fn regsdrtgwraddrackprolongcnt(
        &mut self,
    ) -> RegsdrtgwraddrackprolongcntW<I3cphyctrlreg08cSpec> {
        RegsdrtgwraddrackprolongcntW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_SDR_TG_WR_ADDR_ACK_PROLONG_EN"]
    #[inline(always)]
    pub fn regsdrtgwraddrackprolongen(
        &mut self,
    ) -> RegsdrtgwraddrackprolongenW<I3cphyctrlreg08cSpec> {
        RegsdrtgwraddrackprolongenW::new(self, 31)
    }
}
#[doc = "TG\\_WR\\_ADDR\\_ACK\\_PROLONG\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg08cSpec;
impl crate::RegisterSpec for I3cphyctrlreg08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg08c::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg08cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg08c::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG08C to value 0x11"]
impl crate::Resettable for I3cphyctrlreg08cSpec {
    const RESET_VALUE: u32 = 0x11;
}
