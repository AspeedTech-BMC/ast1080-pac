#[doc = "Register `PRIC_IO408` reader"]
pub type R = crate::R<PricIo408Spec>;
#[doc = "Register `PRIC_IO408` writer"]
pub type W = crate::W<PricIo408Spec>;
#[doc = "Field `RegionNWrMasters` reader - Region #N Write Masters"]
pub type RegionNwrMastersR = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters` writer - Region #N Write Masters"]
pub type RegionNwrMastersW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters(&self) -> RegionNwrMastersR {
        RegionNwrMastersR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters(&mut self) -> RegionNwrMastersW<PricIo408Spec> {
        RegionNwrMastersW::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io408::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io408::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo408Spec;
impl crate::RegisterSpec for PricIo408Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io408::R`](R) reader structure"]
impl crate::Readable for PricIo408Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io408::W`](W) writer structure"]
impl crate::Writable for PricIo408Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO408 to value 0"]
impl crate::Resettable for PricIo408Spec {}
