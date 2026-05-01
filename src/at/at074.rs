#[doc = "Register `AT074` reader"]
pub type R = crate::R<At074Spec>;
#[doc = "Register `AT074` writer"]
pub type W = crate::W<At074Spec>;
#[doc = "Field `ATTDCLKDIV` reader - AT_TD_CLKDIV"]
pub type AttdclkdivR = crate::FieldReader;
#[doc = "Field `ATTDCLKDIV` writer - AT_TD_CLKDIV"]
pub type AttdclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AT_TD_CLKDIV"]
    #[inline(always)]
    pub fn attdclkdiv(&self) -> AttdclkdivR {
        AttdclkdivR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AT_TD_CLKDIV"]
    #[inline(always)]
    pub fn attdclkdiv(&mut self) -> AttdclkdivW<At074Spec> {
        AttdclkdivW::new(self, 0)
    }
}
#[doc = "TSENSE Clock Divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`at074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At074Spec;
impl crate::RegisterSpec for At074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at074::R`](R) reader structure"]
impl crate::Readable for At074Spec {}
#[doc = "`write(|w| ..)` method takes [`at074::W`](W) writer structure"]
impl crate::Writable for At074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT074 to value 0x01"]
impl crate::Resettable for At074Spec {
    const RESET_VALUE: u32 = 0x01;
}
