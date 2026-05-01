#[doc = "Register `AT064` reader"]
pub type R = crate::R<At064Spec>;
#[doc = "Register `AT064` writer"]
pub type W = crate::W<At064Spec>;
#[doc = "Field `ATIRINTSTS` reader - AT_IR_INT_STS"]
pub type AtirintstsR = crate::BitReader;
#[doc = "Field `ATIRINTSTS` writer - AT_IR_INT_STS"]
pub type AtirintstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_IR_INT_STS"]
    #[inline(always)]
    pub fn atirintsts(&self) -> AtirintstsR {
        AtirintstsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_IR_INT_STS"]
    #[inline(always)]
    pub fn atirintsts(&mut self) -> AtirintstsW<At064Spec> {
        AtirintstsW::new(self, 0)
    }
}
#[doc = "IR Drop Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At064Spec;
impl crate::RegisterSpec for At064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at064::R`](R) reader structure"]
impl crate::Readable for At064Spec {}
#[doc = "`write(|w| ..)` method takes [`at064::W`](W) writer structure"]
impl crate::Writable for At064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT064 to value 0"]
impl crate::Resettable for At064Spec {}
