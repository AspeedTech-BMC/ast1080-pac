#[doc = "Register `PRIC_IO4A0` reader"]
pub type R = crate::R<PricIo4a0Spec>;
#[doc = "Register `PRIC_IO4A0` writer"]
pub type W = crate::W<PricIo4a0Spec>;
#[doc = "Field `RegionNEnbl10` reader - Region #N Enable"]
pub type RegionNenbl10R = crate::BitReader;
#[doc = "Field `RegionNEnbl10` writer - Region #N Enable"]
pub type RegionNenbl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `RegionNWrGroup10` reader - Region #N Write Group"]
pub type RegionNwrGroup10R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup10` writer - Region #N Write Group"]
pub type RegionNwrGroup10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr351210` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351210R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr351210` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351210W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl10(&self) -> RegionNenbl10R {
        RegionNenbl10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group10(&self) -> RegionNwrGroup10R {
        RegionNwrGroup10R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351210(&self) -> RegionNstartAddr351210R {
        RegionNstartAddr351210R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl10(&mut self) -> RegionNenbl10W<PricIo4a0Spec> {
        RegionNenbl10W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group10(&mut self) -> RegionNwrGroup10W<PricIo4a0Spec> {
        RegionNwrGroup10W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351210(&mut self) -> RegionNstartAddr351210W<PricIo4a0Spec> {
        RegionNstartAddr351210W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#40\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4a0Spec;
impl crate::RegisterSpec for PricIo4a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4a0::R`](R) reader structure"]
impl crate::Readable for PricIo4a0Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4a0::W`](W) writer structure"]
impl crate::Writable for PricIo4a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4A0 to value 0"]
impl crate::Resettable for PricIo4a0Spec {}
