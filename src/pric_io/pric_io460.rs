#[doc = "Register `PRIC_IO460` reader"]
pub type R = crate::R<PricIo460Spec>;
#[doc = "Register `PRIC_IO460` writer"]
pub type W = crate::W<PricIo460Spec>;
#[doc = "Field `RegionNEnbl6` reader - Region #N Enable"]
pub type RegionNenbl6R = crate::BitReader;
#[doc = "Field `RegionNEnbl6` writer - Region #N Enable"]
pub type RegionNenbl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `RegionNWrGroup6` reader - Region #N Write Group"]
pub type RegionNwrGroup6R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup6` writer - Region #N Write Group"]
pub type RegionNwrGroup6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35126` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35126R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35126` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35126W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl6(&self) -> RegionNenbl6R {
        RegionNenbl6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group6(&self) -> RegionNwrGroup6R {
        RegionNwrGroup6R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35126(&self) -> RegionNstartAddr35126R {
        RegionNstartAddr35126R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl6(&mut self) -> RegionNenbl6W<PricIo460Spec> {
        RegionNenbl6W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group6(&mut self) -> RegionNwrGroup6W<PricIo460Spec> {
        RegionNwrGroup6W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35126(&mut self) -> RegionNstartAddr35126W<PricIo460Spec> {
        RegionNstartAddr35126W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io460::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io460::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo460Spec;
impl crate::RegisterSpec for PricIo460Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io460::R`](R) reader structure"]
impl crate::Readable for PricIo460Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io460::W`](W) writer structure"]
impl crate::Writable for PricIo460Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO460 to value 0"]
impl crate::Resettable for PricIo460Spec {}
