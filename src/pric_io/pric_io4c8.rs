#[doc = "Register `PRIC_IO4C8` reader"]
pub type R = crate::R<PricIo4c8Spec>;
#[doc = "Register `PRIC_IO4C8` writer"]
pub type W = crate::W<PricIo4c8Spec>;
#[doc = "Field `RegionNWrMasters12` reader - Region #N Write Masters"]
pub type RegionNwrMasters12R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters12` writer - Region #N Write Masters"]
pub type RegionNwrMasters12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters12(&self) -> RegionNwrMasters12R {
        RegionNwrMasters12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters12(&mut self) -> RegionNwrMasters12W<PricIo4c8Spec> {
        RegionNwrMasters12W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4c8Spec;
impl crate::RegisterSpec for PricIo4c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4c8::R`](R) reader structure"]
impl crate::Readable for PricIo4c8Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4c8::W`](W) writer structure"]
impl crate::Writable for PricIo4c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4C8 to value 0"]
impl crate::Resettable for PricIo4c8Spec {}
