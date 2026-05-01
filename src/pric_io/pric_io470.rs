#[doc = "Register `PRIC_IO470` reader"]
pub type R = crate::R<PricIo470Spec>;
#[doc = "Register `PRIC_IO470` writer"]
pub type W = crate::W<PricIo470Spec>;
#[doc = "Field `RegionNEnbl7` reader - Region #N Enable"]
pub type RegionNenbl7R = crate::BitReader;
#[doc = "Field `RegionNEnbl7` writer - Region #N Enable"]
pub type RegionNenbl7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RegionNWrGroup7` reader - Region #N Write Group"]
pub type RegionNwrGroup7R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup7` writer - Region #N Write Group"]
pub type RegionNwrGroup7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35127` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35127R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35127` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35127W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl7(&self) -> RegionNenbl7R {
        RegionNenbl7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group7(&self) -> RegionNwrGroup7R {
        RegionNwrGroup7R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35127(&self) -> RegionNstartAddr35127R {
        RegionNstartAddr35127R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl7(&mut self) -> RegionNenbl7W<PricIo470Spec> {
        RegionNenbl7W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group7(&mut self) -> RegionNwrGroup7W<PricIo470Spec> {
        RegionNwrGroup7W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35127(&mut self) -> RegionNstartAddr35127W<PricIo470Spec> {
        RegionNstartAddr35127W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io470::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io470::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo470Spec;
impl crate::RegisterSpec for PricIo470Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io470::R`](R) reader structure"]
impl crate::Readable for PricIo470Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io470::W`](W) writer structure"]
impl crate::Writable for PricIo470Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO470 to value 0"]
impl crate::Resettable for PricIo470Spec {}
