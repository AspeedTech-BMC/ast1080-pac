#[doc = "Register `PRIC_IO40C` reader"]
pub type R = crate::R<PricIo40cSpec>;
#[doc = "Register `PRIC_IO40C` writer"]
pub type W = crate::W<PricIo40cSpec>;
#[doc = "Field `RegionNReadMasters` reader - Region #N Read Masters"]
pub type RegionNreadMastersR = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters` writer - Region #N Read Masters"]
pub type RegionNreadMastersW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters(&self) -> RegionNreadMastersR {
        RegionNreadMastersR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters(&mut self) -> RegionNreadMastersW<PricIo40cSpec> {
        RegionNreadMastersW::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io40c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io40c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo40cSpec;
impl crate::RegisterSpec for PricIo40cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io40c::R`](R) reader structure"]
impl crate::Readable for PricIo40cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io40c::W`](W) writer structure"]
impl crate::Writable for PricIo40cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO40C to value 0"]
impl crate::Resettable for PricIo40cSpec {}
