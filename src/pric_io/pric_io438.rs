#[doc = "Register `PRIC_IO438` reader"]
pub type R = crate::R<PricIo438Spec>;
#[doc = "Register `PRIC_IO438` writer"]
pub type W = crate::W<PricIo438Spec>;
#[doc = "Field `RegionNWrMasters3` reader - Region #N Write Masters"]
pub type RegionNwrMasters3R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters3` writer - Region #N Write Masters"]
pub type RegionNwrMasters3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters3(&self) -> RegionNwrMasters3R {
        RegionNwrMasters3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters3(&mut self) -> RegionNwrMasters3W<PricIo438Spec> {
        RegionNwrMasters3W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io438::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io438::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo438Spec;
impl crate::RegisterSpec for PricIo438Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io438::R`](R) reader structure"]
impl crate::Readable for PricIo438Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io438::W`](W) writer structure"]
impl crate::Writable for PricIo438Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO438 to value 0"]
impl crate::Resettable for PricIo438Spec {}
