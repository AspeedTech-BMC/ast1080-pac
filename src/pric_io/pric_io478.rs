#[doc = "Register `PRIC_IO478` reader"]
pub type R = crate::R<PricIo478Spec>;
#[doc = "Register `PRIC_IO478` writer"]
pub type W = crate::W<PricIo478Spec>;
#[doc = "Field `RegionNWrMasters7` reader - Region #N Write Masters"]
pub type RegionNwrMasters7R = crate::FieldReader<u32>;
#[doc = "Field `RegionNWrMasters7` writer - Region #N Write Masters"]
pub type RegionNwrMasters7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters7(&self) -> RegionNwrMasters7R {
        RegionNwrMasters7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Write Masters"]
    #[inline(always)]
    pub fn region_nwr_masters7(&mut self) -> RegionNwrMasters7W<PricIo478Spec> {
        RegionNwrMasters7W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io478::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io478::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo478Spec;
impl crate::RegisterSpec for PricIo478Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io478::R`](R) reader structure"]
impl crate::Readable for PricIo478Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io478::W`](W) writer structure"]
impl crate::Writable for PricIo478Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO478 to value 0"]
impl crate::Resettable for PricIo478Spec {}
