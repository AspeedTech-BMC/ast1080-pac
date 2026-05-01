#[doc = "Register `PRIC_IO47C` reader"]
pub type R = crate::R<PricIo47cSpec>;
#[doc = "Register `PRIC_IO47C` writer"]
pub type W = crate::W<PricIo47cSpec>;
#[doc = "Field `RegionNReadMasters7` reader - Region #N Read Masters"]
pub type RegionNreadMasters7R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters7` writer - Region #N Read Masters"]
pub type RegionNreadMasters7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters7(&self) -> RegionNreadMasters7R {
        RegionNreadMasters7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters7(&mut self) -> RegionNreadMasters7W<PricIo47cSpec> {
        RegionNreadMasters7W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io47c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io47c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo47cSpec;
impl crate::RegisterSpec for PricIo47cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io47c::R`](R) reader structure"]
impl crate::Readable for PricIo47cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io47c::W`](W) writer structure"]
impl crate::Writable for PricIo47cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO47C to value 0"]
impl crate::Resettable for PricIo47cSpec {}
