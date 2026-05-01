#[doc = "Register `PRIC_IO488` reader"]
pub type R = crate::R<PricIo488Spec>;
#[doc = "Register `PRIC_IO488` writer"]
pub type W = crate::W<PricIo488Spec>;
#[doc = "Field `RegionNWrMasters8` reader - Region #N Write Masters"]
pub type RegionNwrMasters8R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters8` writer - Region #N Write Masters"]
pub type RegionNwrMasters8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters8(&self) -> RegionNwrMasters8R {
        RegionNwrMasters8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters8(&mut self) -> RegionNwrMasters8W<PricIo488Spec> {
        RegionNwrMasters8W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io488::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io488::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo488Spec;
impl crate::RegisterSpec for PricIo488Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io488::R`](R) reader structure"]
impl crate::Readable for PricIo488Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io488::W`](W) writer structure"]
impl crate::Writable for PricIo488Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO488 to value 0"]
impl crate::Resettable for PricIo488Spec {}
