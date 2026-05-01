#[doc = "Register `PRIC_IO428` reader"]
pub type R = crate::R<PricIo428Spec>;
#[doc = "Register `PRIC_IO428` writer"]
pub type W = crate::W<PricIo428Spec>;
#[doc = "Field `RegionNWrMasters2` reader - Region #N Write Masters"]
pub type RegionNwrMasters2R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters2` writer - Region #N Write Masters"]
pub type RegionNwrMasters2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters2(&self) -> RegionNwrMasters2R {
        RegionNwrMasters2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters2(&mut self) -> RegionNwrMasters2W<PricIo428Spec> {
        RegionNwrMasters2W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io428::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io428::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo428Spec;
impl crate::RegisterSpec for PricIo428Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io428::R`](R) reader structure"]
impl crate::Readable for PricIo428Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io428::W`](W) writer structure"]
impl crate::Writable for PricIo428Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO428 to value 0"]
impl crate::Resettable for PricIo428Spec {}
