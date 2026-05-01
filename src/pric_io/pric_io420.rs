#[doc = "Register `PRIC_IO420` reader"]
pub type R = crate::R<PricIo420Spec>;
#[doc = "Register `PRIC_IO420` writer"]
pub type W = crate::W<PricIo420Spec>;
#[doc = "Field `RegionNEnbl2` reader - Region #N Enable"]
pub type RegionNenbl2R = crate::BitReader;
#[doc = "Field `RegionNEnbl2` writer - Region #N Enable"]
pub type RegionNenbl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RegionNWrGroup2` reader - Region #N Write Group"]
pub type RegionNwrGroup2R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup2` writer - Region #N Write Group"]
pub type RegionNwrGroup2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35122` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35122R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35122` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35122W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl2(&self) -> RegionNenbl2R {
        RegionNenbl2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group2(&self) -> RegionNwrGroup2R {
        RegionNwrGroup2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35122(&self) -> RegionNstartAddr35122R {
        RegionNstartAddr35122R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl2(&mut self) -> RegionNenbl2W<PricIo420Spec> {
        RegionNenbl2W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group2(&mut self) -> RegionNwrGroup2W<PricIo420Spec> {
        RegionNwrGroup2W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35122(&mut self) -> RegionNstartAddr35122W<PricIo420Spec> {
        RegionNstartAddr35122W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io420::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io420::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo420Spec;
impl crate::RegisterSpec for PricIo420Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io420::R`](R) reader structure"]
impl crate::Readable for PricIo420Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io420::W`](W) writer structure"]
impl crate::Writable for PricIo420Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO420 to value 0"]
impl crate::Resettable for PricIo420Spec {}
