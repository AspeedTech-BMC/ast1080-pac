#[doc = "Register `I3CPHYCTRLREG084` reader"]
pub type R = crate::R<I3cphyctrlreg084Spec>;
#[doc = "Register `I3CPHYCTRLREG084` writer"]
pub type W = crate::W<I3cphyctrlreg084Spec>;
#[doc = "Field `REGCRHPOVERLAPCNT` reader - REG_CRHP_OVERLAP_CNT"]
pub type RegcrhpoverlapcntR = crate::FieldReader<u16>;
#[doc = "Field `REGCRHPOVERLAPCNT` writer - REG_CRHP_OVERLAP_CNT"]
pub type RegcrhpoverlapcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_CRHP_OVERLAP_CNT"]
    #[inline(always)]
    pub fn regcrhpoverlapcnt(&self) -> RegcrhpoverlapcntR {
        RegcrhpoverlapcntR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_CRHP_OVERLAP_CNT"]
    #[inline(always)]
    pub fn regcrhpoverlapcnt(&mut self) -> RegcrhpoverlapcntW<I3cphyctrlreg084Spec> {
        RegcrhpoverlapcntW::new(self, 0)
    }
}
#[doc = "CCR\\_TO\\_NCR\\_OVERLAP\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg084Spec;
impl crate::RegisterSpec for I3cphyctrlreg084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg084::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg084Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg084::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG084 to value 0x27"]
impl crate::Resettable for I3cphyctrlreg084Spec {
    const RESET_VALUE: u32 = 0x27;
}
