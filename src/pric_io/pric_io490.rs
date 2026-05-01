#[doc = "Register `PRIC_IO490` reader"]
pub type R = crate::R<PricIo490Spec>;
#[doc = "Register `PRIC_IO490` writer"]
pub type W = crate::W<PricIo490Spec>;
#[doc = "Field `RegionNEnbl9` reader - Region #N Enable"]
pub type RegionNenbl9R = crate::BitReader;
#[doc = "Field `RegionNEnbl9` writer - Region #N Enable"]
pub type RegionNenbl9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `RegionNWrGroup9` reader - Region #N Write Group"]
pub type RegionNwrGroup9R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup9` writer - Region #N Write Group"]
pub type RegionNwrGroup9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35129` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35129R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35129` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35129W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl9(&self) -> RegionNenbl9R {
        RegionNenbl9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group9(&self) -> RegionNwrGroup9R {
        RegionNwrGroup9R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35129(&self) -> RegionNstartAddr35129R {
        RegionNstartAddr35129R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl9(&mut self) -> RegionNenbl9W<PricIo490Spec> {
        RegionNenbl9W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group9(&mut self) -> RegionNwrGroup9W<PricIo490Spec> {
        RegionNwrGroup9W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35129(&mut self) -> RegionNstartAddr35129W<PricIo490Spec> {
        RegionNstartAddr35129W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io490::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io490::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo490Spec;
impl crate::RegisterSpec for PricIo490Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io490::R`](R) reader structure"]
impl crate::Readable for PricIo490Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io490::W`](W) writer structure"]
impl crate::Writable for PricIo490Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO490 to value 0"]
impl crate::Resettable for PricIo490Spec {}
