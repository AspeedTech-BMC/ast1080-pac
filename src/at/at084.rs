#[doc = "Register `AT084` reader"]
pub type R = crate::R<At084Spec>;
#[doc = "Register `AT084` writer"]
pub type W = crate::W<At084Spec>;
#[doc = "Field `ATTDINTSTS` reader - AT_TD_INT_STS"]
pub type AttdintstsR = crate::BitReader;
#[doc = "Field `ATTDINTSTS` writer - AT_TD_INT_STS"]
pub type AttdintstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_TD_INT_STS"]
    #[inline(always)]
    pub fn attdintsts(&self) -> AttdintstsR {
        AttdintstsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_TD_INT_STS"]
    #[inline(always)]
    pub fn attdintsts(&mut self) -> AttdintstsW<At084Spec> {
        AttdintstsW::new(self, 0)
    }
}
#[doc = "TSENSE Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At084Spec;
impl crate::RegisterSpec for At084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at084::R`](R) reader structure"]
impl crate::Readable for At084Spec {}
#[doc = "`write(|w| ..)` method takes [`at084::W`](W) writer structure"]
impl crate::Writable for At084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT084 to value 0"]
impl crate::Resettable for At084Spec {}
