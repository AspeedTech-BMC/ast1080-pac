#[doc = "Register `AT068` reader"]
pub type R = crate::R<At068Spec>;
#[doc = "Register `AT068` writer"]
pub type W = crate::W<At068Spec>;
#[doc = "Field `ATIRINTEN` reader - AT_IR_INT_EN"]
pub type AtirintenR = crate::BitReader;
#[doc = "Field `ATIRINTEN` writer - AT_IR_INT_EN"]
pub type AtirintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_IR_INT_EN"]
    #[inline(always)]
    pub fn atirinten(&self) -> AtirintenR {
        AtirintenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_IR_INT_EN"]
    #[inline(always)]
    pub fn atirinten(&mut self) -> AtirintenW<At068Spec> {
        AtirintenW::new(self, 0)
    }
}
#[doc = "IR Drop Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At068Spec;
impl crate::RegisterSpec for At068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at068::R`](R) reader structure"]
impl crate::Readable for At068Spec {}
#[doc = "`write(|w| ..)` method takes [`at068::W`](W) writer structure"]
impl crate::Writable for At068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT068 to value 0"]
impl crate::Resettable for At068Spec {}
