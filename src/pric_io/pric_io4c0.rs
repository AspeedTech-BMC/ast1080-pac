#[doc = "Register `PRIC_IO4C0` reader"]
pub type R = crate::R<PricIo4c0Spec>;
#[doc = "Register `PRIC_IO4C0` writer"]
pub type W = crate::W<PricIo4c0Spec>;
#[doc = "Field `RegionNEnbl12` reader - Region #N Enable"]
pub type RegionNenbl12R = crate::BitReader;
#[doc = "Field `RegionNEnbl12` writer - Region #N Enable"]
pub type RegionNenbl12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `RegionNWrGroup12` reader - Region #N Write Group"]
pub type RegionNwrGroup12R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup12` writer - Region #N Write Group"]
pub type RegionNwrGroup12W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr351212` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351212R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr351212` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351212W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl12(&self) -> RegionNenbl12R {
        RegionNenbl12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group12(&self) -> RegionNwrGroup12R {
        RegionNwrGroup12R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351212(&self) -> RegionNstartAddr351212R {
        RegionNstartAddr351212R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl12(&mut self) -> RegionNenbl12W<PricIo4c0Spec> {
        RegionNenbl12W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group12(&mut self) -> RegionNwrGroup12W<PricIo4c0Spec> {
        RegionNwrGroup12W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351212(&mut self) -> RegionNstartAddr351212W<PricIo4c0Spec> {
        RegionNstartAddr351212W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4c0Spec;
impl crate::RegisterSpec for PricIo4c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4c0::R`](R) reader structure"]
impl crate::Readable for PricIo4c0Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4c0::W`](W) writer structure"]
impl crate::Writable for PricIo4c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4C0 to value 0"]
impl crate::Resettable for PricIo4c0Spec {}
