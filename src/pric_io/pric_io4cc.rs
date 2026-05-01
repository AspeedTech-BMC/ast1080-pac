#[doc = "Register `PRIC_IO4CC` reader"]
pub type R = crate::R<PricIo4ccSpec>;
#[doc = "Register `PRIC_IO4CC` writer"]
pub type W = crate::W<PricIo4ccSpec>;
#[doc = "Field `RegionNReadMasters12` reader - Region #N Read Masters"]
pub type RegionNreadMasters12R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters12` writer - Region #N Read Masters"]
pub type RegionNreadMasters12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters12(&self) -> RegionNreadMasters12R {
        RegionNreadMasters12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters12(&mut self) -> RegionNreadMasters12W<PricIo4ccSpec> {
        RegionNreadMasters12W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4ccSpec;
impl crate::RegisterSpec for PricIo4ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4cc::R`](R) reader structure"]
impl crate::Readable for PricIo4ccSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4cc::W`](W) writer structure"]
impl crate::Writable for PricIo4ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4CC to value 0"]
impl crate::Resettable for PricIo4ccSpec {}
