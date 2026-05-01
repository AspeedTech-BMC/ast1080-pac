#[doc = "Register `PRIC_IO4D8` reader"]
pub type R = crate::R<PricIo4d8Spec>;
#[doc = "Register `PRIC_IO4D8` writer"]
pub type W = crate::W<PricIo4d8Spec>;
#[doc = "Field `RegionNWrMasters13` reader - Region #N Write Masters"]
pub type RegionNwrMasters13R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters13` writer - Region #N Write Masters"]
pub type RegionNwrMasters13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters13(&self) -> RegionNwrMasters13R {
        RegionNwrMasters13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters13(&mut self) -> RegionNwrMasters13W<PricIo4d8Spec> {
        RegionNwrMasters13W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#54\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4d8Spec;
impl crate::RegisterSpec for PricIo4d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4d8::R`](R) reader structure"]
impl crate::Readable for PricIo4d8Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4d8::W`](W) writer structure"]
impl crate::Writable for PricIo4d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4D8 to value 0"]
impl crate::Resettable for PricIo4d8Spec {}
