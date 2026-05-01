#[doc = "Register `PRIC_IO42C` reader"]
pub type R = crate::R<PricIo42cSpec>;
#[doc = "Register `PRIC_IO42C` writer"]
pub type W = crate::W<PricIo42cSpec>;
#[doc = "Field `RegionNReadMasters2` reader - Region #N Read Masters"]
pub type RegionNreadMasters2R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters2` writer - Region #N Read Masters"]
pub type RegionNreadMasters2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters2(&self) -> RegionNreadMasters2R {
        RegionNreadMasters2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters2(&mut self) -> RegionNreadMasters2W<PricIo42cSpec> {
        RegionNreadMasters2W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io42c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io42c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo42cSpec;
impl crate::RegisterSpec for PricIo42cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io42c::R`](R) reader structure"]
impl crate::Readable for PricIo42cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io42c::W`](W) writer structure"]
impl crate::Writable for PricIo42cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO42C to value 0"]
impl crate::Resettable for PricIo42cSpec {}
