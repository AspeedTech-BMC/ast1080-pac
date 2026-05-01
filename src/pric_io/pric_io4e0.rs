#[doc = "Register `PRIC_IO4E0` reader"]
pub type R = crate::R<PricIo4e0Spec>;
#[doc = "Register `PRIC_IO4E0` writer"]
pub type W = crate::W<PricIo4e0Spec>;
#[doc = "Field `RegionNEnbl14` reader - Region #N Enable"]
pub type RegionNenbl14R = crate::BitReader;
#[doc = "Field `RegionNEnbl14` writer - Region #N Enable"]
pub type RegionNenbl14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `RegionNWrGroup14` reader - Region #N Write Group"]
pub type RegionNwrGroup14R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup14` writer - Region #N Write Group"]
pub type RegionNwrGroup14W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr351214` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351214R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr351214` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351214W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl14(&self) -> RegionNenbl14R {
        RegionNenbl14R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group14(&self) -> RegionNwrGroup14R {
        RegionNwrGroup14R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351214(&self) -> RegionNstartAddr351214R {
        RegionNstartAddr351214R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl14(&mut self) -> RegionNenbl14W<PricIo4e0Spec> {
        RegionNenbl14W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group14(&mut self) -> RegionNwrGroup14W<PricIo4e0Spec> {
        RegionNwrGroup14W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351214(&mut self) -> RegionNstartAddr351214W<PricIo4e0Spec> {
        RegionNstartAddr351214W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#56\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4e0Spec;
impl crate::RegisterSpec for PricIo4e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4e0::R`](R) reader structure"]
impl crate::Readable for PricIo4e0Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4e0::W`](W) writer structure"]
impl crate::Writable for PricIo4e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4E0 to value 0"]
impl crate::Resettable for PricIo4e0Spec {}
