#[doc = "Register `AT044` reader"]
pub type R = crate::R<At044Spec>;
#[doc = "Register `AT044` writer"]
pub type W = crate::W<At044Spec>;
#[doc = "Field `ATGDLOWINTSTS` reader - AT_GD_LOW_INT_STS"]
pub type AtgdlowintstsR = crate::BitReader;
#[doc = "Field `ATGDLOWINTSTS` writer - AT_GD_LOW_INT_STS"]
pub type AtgdlowintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDHIGHINTSTS` reader - AT_GD_HIGH_INT_STS"]
pub type AtgdhighintstsR = crate::BitReader;
#[doc = "Field `ATGDHIGHINTSTS` writer - AT_GD_HIGH_INT_STS"]
pub type AtgdhighintstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_GD_LOW_INT_STS"]
    #[inline(always)]
    pub fn atgdlowintsts(&self) -> AtgdlowintstsR {
        AtgdlowintstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_GD_HIGH_INT_STS"]
    #[inline(always)]
    pub fn atgdhighintsts(&self) -> AtgdhighintstsR {
        AtgdhighintstsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_GD_LOW_INT_STS"]
    #[inline(always)]
    pub fn atgdlowintsts(&mut self) -> AtgdlowintstsW<At044Spec> {
        AtgdlowintstsW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_GD_HIGH_INT_STS"]
    #[inline(always)]
    pub fn atgdhighintsts(&mut self) -> AtgdhighintstsW<At044Spec> {
        AtgdhighintstsW::new(self, 1)
    }
}
#[doc = "Glitch Detection Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`at044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At044Spec;
impl crate::RegisterSpec for At044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at044::R`](R) reader structure"]
impl crate::Readable for At044Spec {}
#[doc = "`write(|w| ..)` method takes [`at044::W`](W) writer structure"]
impl crate::Writable for At044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT044 to value 0"]
impl crate::Resettable for At044Spec {}
