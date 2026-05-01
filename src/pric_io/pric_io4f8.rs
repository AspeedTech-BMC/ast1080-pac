#[doc = "Register `PRIC_IO4F8` reader"]
pub type R = crate::R<PricIo4f8Spec>;
#[doc = "Register `PRIC_IO4F8` writer"]
pub type W = crate::W<PricIo4f8Spec>;
#[doc = "Field `RegionNWrMasters15` reader - Region #N Write Masters"]
pub type RegionNwrMasters15R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters15` writer - Region #N Write Masters"]
pub type RegionNwrMasters15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters15(&self) -> RegionNwrMasters15R {
        RegionNwrMasters15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters15(&mut self) -> RegionNwrMasters15W<PricIo4f8Spec> {
        RegionNwrMasters15W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#62\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4f8Spec;
impl crate::RegisterSpec for PricIo4f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4f8::R`](R) reader structure"]
impl crate::Readable for PricIo4f8Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4f8::W`](W) writer structure"]
impl crate::Writable for PricIo4f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4F8 to value 0"]
impl crate::Resettable for PricIo4f8Spec {}
