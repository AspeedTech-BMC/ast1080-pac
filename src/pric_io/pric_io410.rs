#[doc = "Register `PRIC_IO410` reader"]
pub type R = crate::R<PricIo410Spec>;
#[doc = "Register `PRIC_IO410` writer"]
pub type W = crate::W<PricIo410Spec>;
#[doc = "Field `RegionNEnbl1` reader - Region #N Enable"]
pub type RegionNenbl1R = crate::BitReader;
#[doc = "Field `RegionNEnbl1` writer - Region #N Enable"]
pub type RegionNenbl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RegionNWrGroup1` reader - Region #N Write Group"]
pub type RegionNwrGroup1R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup1` writer - Region #N Write Group"]
pub type RegionNwrGroup1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35121` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35121R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35121` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35121W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl1(&self) -> RegionNenbl1R {
        RegionNenbl1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group1(&self) -> RegionNwrGroup1R {
        RegionNwrGroup1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35121(&self) -> RegionNstartAddr35121R {
        RegionNstartAddr35121R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl1(&mut self) -> RegionNenbl1W<PricIo410Spec> {
        RegionNenbl1W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group1(&mut self) -> RegionNwrGroup1W<PricIo410Spec> {
        RegionNwrGroup1W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35121(&mut self) -> RegionNstartAddr35121W<PricIo410Spec> {
        RegionNstartAddr35121W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io410::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io410::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo410Spec;
impl crate::RegisterSpec for PricIo410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io410::R`](R) reader structure"]
impl crate::Readable for PricIo410Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io410::W`](W) writer structure"]
impl crate::Writable for PricIo410Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO410 to value 0"]
impl crate::Resettable for PricIo410Spec {}
