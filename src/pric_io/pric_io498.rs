#[doc = "Register `PRIC_IO498` reader"]
pub type R = crate::R<PricIo498Spec>;
#[doc = "Register `PRIC_IO498` writer"]
pub type W = crate::W<PricIo498Spec>;
#[doc = "Field `RegionNWrMasters9` reader - Region #N Write Masters"]
pub type RegionNwrMasters9R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters9` writer - Region #N Write Masters"]
pub type RegionNwrMasters9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters9(&self) -> RegionNwrMasters9R {
        RegionNwrMasters9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters9(&mut self) -> RegionNwrMasters9W<PricIo498Spec> {
        RegionNwrMasters9W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io498::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io498::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo498Spec;
impl crate::RegisterSpec for PricIo498Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io498::R`](R) reader structure"]
impl crate::Readable for PricIo498Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io498::W`](W) writer structure"]
impl crate::Writable for PricIo498Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO498 to value 0"]
impl crate::Resettable for PricIo498Spec {}
