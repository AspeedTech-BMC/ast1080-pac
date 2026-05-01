#[doc = "Register `AT048` reader"]
pub type R = crate::R<At048Spec>;
#[doc = "Register `AT048` writer"]
pub type W = crate::W<At048Spec>;
#[doc = "Field `ATGDLOWINTEN` reader - AT_GD_LOW_INT_EN"]
pub type AtgdlowintenR = crate::BitReader;
#[doc = "Field `ATGDLOWINTEN` writer - AT_GD_LOW_INT_EN"]
pub type AtgdlowintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDHIGHINTEN` reader - AT_GD_HIGH_INT_EN"]
pub type AtgdhighintenR = crate::BitReader;
#[doc = "Field `ATGDHIGHINTEN` writer - AT_GD_HIGH_INT_EN"]
pub type AtgdhighintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AT_GD_LOW_INT_EN"]
    #[inline(always)]
    pub fn atgdlowinten(&self) -> AtgdlowintenR {
        AtgdlowintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_GD_HIGH_INT_EN"]
    #[inline(always)]
    pub fn atgdhighinten(&self) -> AtgdhighintenR {
        AtgdhighintenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AT_GD_LOW_INT_EN"]
    #[inline(always)]
    pub fn atgdlowinten(&mut self) -> AtgdlowintenW<At048Spec> {
        AtgdlowintenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_GD_HIGH_INT_EN"]
    #[inline(always)]
    pub fn atgdhighinten(&mut self) -> AtgdhighintenW<At048Spec> {
        AtgdhighintenW::new(self, 1)
    }
}
#[doc = "Glitch Detection Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`at048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At048Spec;
impl crate::RegisterSpec for At048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at048::R`](R) reader structure"]
impl crate::Readable for At048Spec {}
#[doc = "`write(|w| ..)` method takes [`at048::W`](W) writer structure"]
impl crate::Writable for At048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT048 to value 0"]
impl crate::Resettable for At048Spec {}
