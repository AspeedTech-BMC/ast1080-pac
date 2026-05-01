#[doc = "Register `PRIC_IO45C` reader"]
pub type R = crate::R<PricIo45cSpec>;
#[doc = "Register `PRIC_IO45C` writer"]
pub type W = crate::W<PricIo45cSpec>;
#[doc = "Field `RegionNReadMasters5` reader - Region #N Read Masters"]
pub type RegionNreadMasters5R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters5` writer - Region #N Read Masters"]
pub type RegionNreadMasters5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters5(&self) -> RegionNreadMasters5R {
        RegionNreadMasters5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters5(&mut self) -> RegionNreadMasters5W<PricIo45cSpec> {
        RegionNreadMasters5W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io45c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io45c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo45cSpec;
impl crate::RegisterSpec for PricIo45cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io45c::R`](R) reader structure"]
impl crate::Readable for PricIo45cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io45c::W`](W) writer structure"]
impl crate::Writable for PricIo45cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO45C to value 0"]
impl crate::Resettable for PricIo45cSpec {}
