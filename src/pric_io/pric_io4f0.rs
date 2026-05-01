#[doc = "Register `PRIC_IO4F0` reader"]
pub type R = crate::R<PricIo4f0Spec>;
#[doc = "Register `PRIC_IO4F0` writer"]
pub type W = crate::W<PricIo4f0Spec>;
#[doc = "Field `RegionNEnbl15` reader - Region #N Enable"]
pub type RegionNenbl15R = crate::BitReader;
#[doc = "Field `RegionNEnbl15` writer - Region #N Enable"]
pub type RegionNenbl15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `RegionNWrGroup15` reader - Region #N Write Group"]
pub type RegionNwrGroup15R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup15` writer - Region #N Write Group"]
pub type RegionNwrGroup15W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr351215` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351215R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr351215` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351215W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl15(&self) -> RegionNenbl15R {
        RegionNenbl15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group15(&self) -> RegionNwrGroup15R {
        RegionNwrGroup15R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351215(&self) -> RegionNstartAddr351215R {
        RegionNstartAddr351215R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl15(&mut self) -> RegionNenbl15W<PricIo4f0Spec> {
        RegionNenbl15W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group15(&mut self) -> RegionNwrGroup15W<PricIo4f0Spec> {
        RegionNwrGroup15W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351215(&mut self) -> RegionNstartAddr351215W<PricIo4f0Spec> {
        RegionNstartAddr351215W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#60\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4f0Spec;
impl crate::RegisterSpec for PricIo4f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4f0::R`](R) reader structure"]
impl crate::Readable for PricIo4f0Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4f0::W`](W) writer structure"]
impl crate::Writable for PricIo4f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4F0 to value 0"]
impl crate::Resettable for PricIo4f0Spec {}
