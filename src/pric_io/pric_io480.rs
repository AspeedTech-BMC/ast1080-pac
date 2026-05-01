#[doc = "Register `PRIC_IO480` reader"]
pub type R = crate::R<PricIo480Spec>;
#[doc = "Register `PRIC_IO480` writer"]
pub type W = crate::W<PricIo480Spec>;
#[doc = "Field `RegionNEnbl8` reader - Region #N Enable"]
pub type RegionNenbl8R = crate::BitReader;
#[doc = "Field `RegionNEnbl8` writer - Region #N Enable"]
pub type RegionNenbl8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `RegionNWrGroup8` reader - Region #N Write Group"]
pub type RegionNwrGroup8R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup8` writer - Region #N Write Group"]
pub type RegionNwrGroup8W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35128` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35128R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35128` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35128W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl8(&self) -> RegionNenbl8R {
        RegionNenbl8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group8(&self) -> RegionNwrGroup8R {
        RegionNwrGroup8R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35128(&self) -> RegionNstartAddr35128R {
        RegionNstartAddr35128R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl8(&mut self) -> RegionNenbl8W<PricIo480Spec> {
        RegionNenbl8W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group8(&mut self) -> RegionNwrGroup8W<PricIo480Spec> {
        RegionNwrGroup8W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35128(&mut self) -> RegionNstartAddr35128W<PricIo480Spec> {
        RegionNstartAddr35128W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io480::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io480::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo480Spec;
impl crate::RegisterSpec for PricIo480Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io480::R`](R) reader structure"]
impl crate::Readable for PricIo480Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io480::W`](W) writer structure"]
impl crate::Writable for PricIo480Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO480 to value 0"]
impl crate::Resettable for PricIo480Spec {}
