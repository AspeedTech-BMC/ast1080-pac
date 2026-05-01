#[doc = "Register `I3CPHYCTRLREG094` reader"]
pub type R = crate::R<I3cphyctrlreg094Spec>;
#[doc = "Register `I3CPHYCTRLREG094` writer"]
pub type W = crate::W<I3cphyctrlreg094Spec>;
#[doc = "Field `REGDDREARLYTMCNT` reader - REG_DDR_EARLY_TM_CNT"]
pub type RegddrearlytmcntR = crate::FieldReader;
#[doc = "Field `REGDDREARLYTMCNT` writer - REG_DDR_EARLY_TM_CNT"]
pub type RegddrearlytmcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGDDRHANDOFFCNT` reader - REG_DDR_HANDOFF_CNT"]
pub type RegddrhandoffcntR = crate::FieldReader;
#[doc = "Field `REGDDRHANDOFFCNT` writer - REG_DDR_HANDOFF_CNT"]
pub type RegddrhandoffcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - REG_DDR_EARLY_TM_CNT"]
    #[inline(always)]
    pub fn regddrearlytmcnt(&self) -> RegddrearlytmcntR {
        RegddrearlytmcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_DDR_HANDOFF_CNT"]
    #[inline(always)]
    pub fn regddrhandoffcnt(&self) -> RegddrhandoffcntR {
        RegddrhandoffcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_DDR_EARLY_TM_CNT"]
    #[inline(always)]
    pub fn regddrearlytmcnt(&mut self) -> RegddrearlytmcntW<I3cphyctrlreg094Spec> {
        RegddrearlytmcntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_DDR_HANDOFF_CNT"]
    #[inline(always)]
    pub fn regddrhandoffcnt(&mut self) -> RegddrhandoffcntW<I3cphyctrlreg094Spec> {
        RegddrhandoffcntW::new(self, 8)
    }
}
#[doc = "DDR\\_CMD\\_HANDOFF\\_EARLY\\_TM\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg094Spec;
impl crate::RegisterSpec for I3cphyctrlreg094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg094::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg094Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg094::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG094 to value 0x0407"]
impl crate::Resettable for I3cphyctrlreg094Spec {
    const RESET_VALUE: u32 = 0x0407;
}
