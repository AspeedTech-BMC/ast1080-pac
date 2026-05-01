#[doc = "Register `PRIC_IO430` reader"]
pub type R = crate::R<PricIo430Spec>;
#[doc = "Register `PRIC_IO430` writer"]
pub type W = crate::W<PricIo430Spec>;
#[doc = "Field `RegionNEnbl3` reader - Region #N Enable"]
pub type RegionNenbl3R = crate::BitReader;
#[doc = "Field `RegionNEnbl3` writer - Region #N Enable"]
pub type RegionNenbl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RegionNWrGroup3` reader - Region #N Write Group"]
pub type RegionNwrGroup3R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup3` writer - Region #N Write Group"]
pub type RegionNwrGroup3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35123` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35123R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35123` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35123W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl3(&self) -> RegionNenbl3R {
        RegionNenbl3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group3(&self) -> RegionNwrGroup3R {
        RegionNwrGroup3R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35123(&self) -> RegionNstartAddr35123R {
        RegionNstartAddr35123R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl3(&mut self) -> RegionNenbl3W<PricIo430Spec> {
        RegionNenbl3W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group3(&mut self) -> RegionNwrGroup3W<PricIo430Spec> {
        RegionNwrGroup3W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35123(&mut self) -> RegionNstartAddr35123W<PricIo430Spec> {
        RegionNstartAddr35123W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io430::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io430::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo430Spec;
impl crate::RegisterSpec for PricIo430Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io430::R`](R) reader structure"]
impl crate::Readable for PricIo430Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io430::W`](W) writer structure"]
impl crate::Writable for PricIo430Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO430 to value 0"]
impl crate::Resettable for PricIo430Spec {}
