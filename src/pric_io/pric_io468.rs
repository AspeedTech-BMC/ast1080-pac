#[doc = "Register `PRIC_IO468` reader"]
pub type R = crate::R<PricIo468Spec>;
#[doc = "Register `PRIC_IO468` writer"]
pub type W = crate::W<PricIo468Spec>;
#[doc = "Field `RegionNWrMasters6` reader - Region #N Write Masters"]
pub type RegionNwrMasters6R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters6` writer - Region #N Write Masters"]
pub type RegionNwrMasters6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters6(&self) -> RegionNwrMasters6R {
        RegionNwrMasters6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters6(&mut self) -> RegionNwrMasters6W<PricIo468Spec> {
        RegionNwrMasters6W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#26\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io468::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io468::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo468Spec;
impl crate::RegisterSpec for PricIo468Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io468::R`](R) reader structure"]
impl crate::Readable for PricIo468Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io468::W`](W) writer structure"]
impl crate::Writable for PricIo468Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO468 to value 0"]
impl crate::Resettable for PricIo468Spec {}
