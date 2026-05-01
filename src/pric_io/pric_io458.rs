#[doc = "Register `PRIC_IO458` reader"]
pub type R = crate::R<PricIo458Spec>;
#[doc = "Register `PRIC_IO458` writer"]
pub type W = crate::W<PricIo458Spec>;
#[doc = "Field `RegionNWrMasters5` reader - Region #N Write Masters"]
pub type RegionNwrMasters5R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters5` writer - Region #N Write Masters"]
pub type RegionNwrMasters5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters5(&self) -> RegionNwrMasters5R {
        RegionNwrMasters5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters5(&mut self) -> RegionNwrMasters5W<PricIo458Spec> {
        RegionNwrMasters5W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io458::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io458::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo458Spec;
impl crate::RegisterSpec for PricIo458Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io458::R`](R) reader structure"]
impl crate::Readable for PricIo458Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io458::W`](W) writer structure"]
impl crate::Writable for PricIo458Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO458 to value 0"]
impl crate::Resettable for PricIo458Spec {}
