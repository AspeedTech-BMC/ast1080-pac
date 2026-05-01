#[doc = "Register `PRIC_IO46C` reader"]
pub type R = crate::R<PricIo46cSpec>;
#[doc = "Register `PRIC_IO46C` writer"]
pub type W = crate::W<PricIo46cSpec>;
#[doc = "Field `RegionNReadMasters6` reader - Region #N Read Masters"]
pub type RegionNreadMasters6R = crate::FieldReader<u32>;
#[doc = "Field `RegionNReadMasters6` writer - Region #N Read Masters"]
pub type RegionNreadMasters6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters6(&self) -> RegionNreadMasters6R {
        RegionNreadMasters6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region #N Read Masters"]
    #[inline(always)]
    pub fn region_nread_masters6(&mut self) -> RegionNreadMasters6W<PricIo46cSpec> {
        RegionNreadMasters6W::new(self, 0)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io46c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io46c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo46cSpec;
impl crate::RegisterSpec for PricIo46cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io46c::R`](R) reader structure"]
impl crate::Readable for PricIo46cSpec {}
#[doc = "`write(|w| ..)` method takes [`pric_io46c::W`](W) writer structure"]
impl crate::Writable for PricIo46cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO46C to value 0"]
impl crate::Resettable for PricIo46cSpec {}
