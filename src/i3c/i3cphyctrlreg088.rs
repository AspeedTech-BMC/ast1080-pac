#[doc = "Register `I3CPHYCTRLREG088` reader"]
pub type R = crate::R<I3cphyctrlreg088Spec>;
#[doc = "Register `I3CPHYCTRLREG088` writer"]
pub type W = crate::W<I3cphyctrlreg088Spec>;
#[doc = "Field `REGIBIADDRACKPROLONGCNT` reader - REG_IBI_ADDR_ACK_PROLONG_CNT"]
pub type RegibiaddrackprolongcntR = crate::FieldReader<u16>;
#[doc = "Field `REGIBIADDRACKPROLONGCNT` writer - REG_IBI_ADDR_ACK_PROLONG_CNT"]
pub type RegibiaddrackprolongcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_IBI_ADDR_ACK_PROLONG_CNT"]
    #[inline(always)]
    pub fn regibiaddrackprolongcnt(&self) -> RegibiaddrackprolongcntR {
        RegibiaddrackprolongcntR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_IBI_ADDR_ACK_PROLONG_CNT"]
    #[inline(always)]
    pub fn regibiaddrackprolongcnt(&mut self) -> RegibiaddrackprolongcntW<I3cphyctrlreg088Spec> {
        RegibiaddrackprolongcntW::new(self, 0)
    }
}
#[doc = "CR\\_IBI\\_ADDR\\_ACK\\_PROLONG\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg088Spec;
impl crate::RegisterSpec for I3cphyctrlreg088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg088::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg088Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg088::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG088 to value 0x04"]
impl crate::Resettable for I3cphyctrlreg088Spec {
    const RESET_VALUE: u32 = 0x04;
}
