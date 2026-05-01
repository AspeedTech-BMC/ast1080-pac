#[doc = "Register `PRIC_IO448` reader"]
pub type R = crate::R<PricIo448Spec>;
#[doc = "Register `PRIC_IO448` writer"]
pub type W = crate::W<PricIo448Spec>;
#[doc = "Field `RegionNWrMasters4` reader - Region #N Write Masters"]
pub type RegionNwrMasters4R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters4` writer - Region #N Write Masters"]
pub type RegionNwrMasters4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters4(&self) -> RegionNwrMasters4R {
        RegionNwrMasters4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters4(&mut self) -> RegionNwrMasters4W<PricIo448Spec> {
        RegionNwrMasters4W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io448::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io448::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo448Spec;
impl crate::RegisterSpec for PricIo448Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io448::R`](R) reader structure"]
impl crate::Readable for PricIo448Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io448::W`](W) writer structure"]
impl crate::Writable for PricIo448Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO448 to value 0"]
impl crate::Resettable for PricIo448Spec {}
