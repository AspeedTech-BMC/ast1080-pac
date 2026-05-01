#[doc = "Register `PRIC_IO400` reader"]
pub type R = crate::R<PricIo400Spec>;
#[doc = "Register `PRIC_IO400` writer"]
pub type W = crate::W<PricIo400Spec>;
#[doc = "Field `RegionNEnbl` reader - Region #N Enable"]
pub type RegionNenblR = crate::BitReader;
#[doc = "Field `RegionNEnbl` writer - Region #N Enable"]
pub type RegionNenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RegionNWrGroup` reader - Region #N Write Group"]
pub type RegionNwrGroupR = crate::FieldReader;
#[doc = "Field `RegionNWrGroup` writer - Region #N Write Group"]
pub type RegionNwrGroupW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr3512` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr3512R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr3512` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr3512W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl(&self) -> RegionNenblR {
        RegionNenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group(&self) -> RegionNwrGroupR {
        RegionNwrGroupR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr3512(&self) -> RegionNstartAddr3512R {
        RegionNstartAddr3512R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl(&mut self) -> RegionNenblW<PricIo400Spec> {
        RegionNenblW::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group(&mut self) -> RegionNwrGroupW<PricIo400Spec> {
        RegionNwrGroupW::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr3512(&mut self) -> RegionNstartAddr3512W<PricIo400Spec> {
        RegionNstartAddr3512W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io400::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io400::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo400Spec;
impl crate::RegisterSpec for PricIo400Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io400::R`](R) reader structure"]
impl crate::Readable for PricIo400Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io400::W`](W) writer structure"]
impl crate::Writable for PricIo400Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO400 to value 0"]
impl crate::Resettable for PricIo400Spec {}
