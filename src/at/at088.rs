#[doc = "Register `AT088` reader"]
pub type R = crate::R<At088Spec>;
#[doc = "Register `AT088` writer"]
pub type W = crate::W<At088Spec>;
#[doc = "Field `ATTDINTEN` reader - AT_TD_INT_EN"]
pub type AttdintenR = crate::BitReader;
#[doc = "Field `ATTDINTEN` writer - AT_TD_INT_EN"]
pub type AttdintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_TD_INT_EN"]
    #[inline(always)]
    pub fn attdinten(&self) -> AttdintenR {
        AttdintenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_TD_INT_EN"]
    #[inline(always)]
    pub fn attdinten(&mut self) -> AttdintenW<At088Spec> {
        AttdintenW::new(self, 0)
    }
}
#[doc = "TSENSE Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At088Spec;
impl crate::RegisterSpec for At088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at088::R`](R) reader structure"]
impl crate::Readable for At088Spec {}
#[doc = "`write(|w| ..)` method takes [`at088::W`](W) writer structure"]
impl crate::Writable for At088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT088 to value 0"]
impl crate::Resettable for At088Spec {}
