#[doc = "Register `PRIC_IO440` reader"]
pub type R = crate::R<PricIo440Spec>;
#[doc = "Register `PRIC_IO440` writer"]
pub type W = crate::W<PricIo440Spec>;
#[doc = "Field `RegionNEnbl4` reader - Region #N Enable"]
pub type RegionNenbl4R = crate::BitReader;
#[doc = "Field `RegionNEnbl4` writer - Region #N Enable"]
pub type RegionNenbl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `RegionNWrGroup4` reader - Region #N Write Group"]
pub type RegionNwrGroup4R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup4` writer - Region #N Write Group"]
pub type RegionNwrGroup4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35124` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35124R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35124` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35124W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl4(&self) -> RegionNenbl4R {
        RegionNenbl4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group4(&self) -> RegionNwrGroup4R {
        RegionNwrGroup4R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35124(&self) -> RegionNstartAddr35124R {
        RegionNstartAddr35124R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl4(&mut self) -> RegionNenbl4W<PricIo440Spec> {
        RegionNenbl4W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group4(&mut self) -> RegionNwrGroup4W<PricIo440Spec> {
        RegionNwrGroup4W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35124(&mut self) -> RegionNstartAddr35124W<PricIo440Spec> {
        RegionNstartAddr35124W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io440::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io440::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo440Spec;
impl crate::RegisterSpec for PricIo440Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io440::R`](R) reader structure"]
impl crate::Readable for PricIo440Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io440::W`](W) writer structure"]
impl crate::Writable for PricIo440Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO440 to value 0"]
impl crate::Resettable for PricIo440Spec {}
