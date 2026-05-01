#[doc = "Register `PRIC_IO450` reader"]
pub type R = crate::R<PricIo450Spec>;
#[doc = "Register `PRIC_IO450` writer"]
pub type W = crate::W<PricIo450Spec>;
#[doc = "Field `RegionNEnbl5` reader - Region #N Enable"]
pub type RegionNenbl5R = crate::BitReader;
#[doc = "Field `RegionNEnbl5` writer - Region #N Enable"]
pub type RegionNenbl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `RegionNWrGroup5` reader - Region #N Write Group"]
pub type RegionNwrGroup5R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup5` writer - Region #N Write Group"]
pub type RegionNwrGroup5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr35125` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35125R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr35125` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr35125W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl5(&self) -> RegionNenbl5R {
        RegionNenbl5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group5(&self) -> RegionNwrGroup5R {
        RegionNwrGroup5R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35125(&self) -> RegionNstartAddr35125R {
        RegionNstartAddr35125R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl5(&mut self) -> RegionNenbl5W<PricIo450Spec> {
        RegionNenbl5W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group5(&mut self) -> RegionNwrGroup5W<PricIo450Spec> {
        RegionNwrGroup5W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr35125(&mut self) -> RegionNstartAddr35125W<PricIo450Spec> {
        RegionNstartAddr35125W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io450::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io450::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo450Spec;
impl crate::RegisterSpec for PricIo450Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io450::R`](R) reader structure"]
impl crate::Readable for PricIo450Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io450::W`](W) writer structure"]
impl crate::Writable for PricIo450Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO450 to value 0"]
impl crate::Resettable for PricIo450Spec {}
