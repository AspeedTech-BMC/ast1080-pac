#[doc = "Register `PRIC_IO4B0` reader"]
pub type R = crate::R<PricIo4b0Spec>;
#[doc = "Register `PRIC_IO4B0` writer"]
pub type W = crate::W<PricIo4b0Spec>;
#[doc = "Field `RegionNEnbl11` reader - Region #N Enable"]
pub type RegionNenbl11R = crate::BitReader;
#[doc = "Field `RegionNEnbl11` writer - Region #N Enable"]
pub type RegionNenbl11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `RegionNWrGroup11` reader - Region #N Write Group"]
pub type RegionNwrGroup11R = crate::FieldReader;
#[doc = "Field `RegionNWrGroup11` writer - Region #N Write Group"]
pub type RegionNwrGroup11W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RegionNStartAddr351211` reader - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351211R = crate::FieldReader<u32>;
#[doc = "Field `RegionNStartAddr351211` writer - Region #N Start Address\\[35:12\\]"]
pub type RegionNstartAddr351211W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl11(&self) -> RegionNenbl11R {
        RegionNenbl11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group11(&self) -> RegionNwrGroup11R {
        RegionNwrGroup11R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351211(&self) -> RegionNstartAddr351211R {
        RegionNstartAddr351211R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Region #N Enable"]
    #[inline(always)]
    pub fn region_nenbl11(&mut self) -> RegionNenbl11W<PricIo4b0Spec> {
        RegionNenbl11W::new(self, 0)
    }
    #[doc = "Bits 2:7 - Region #N Write Group"]
    #[inline(always)]
    pub fn region_nwr_group11(&mut self) -> RegionNwrGroup11W<PricIo4b0Spec> {
        RegionNwrGroup11W::new(self, 2)
    }
    #[doc = "Bits 8:31 - Region #N Start Address\\[35:12\\]"]
    #[inline(always)]
    pub fn region_nstart_addr351211(&mut self) -> RegionNstartAddr351211W<PricIo4b0Spec> {
        RegionNstartAddr351211W::new(self, 8)
    }
}
#[doc = "Memory Region Proection for AHB Masters Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io4b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io4b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo4b0Spec;
impl crate::RegisterSpec for PricIo4b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io4b0::R`](R) reader structure"]
impl crate::Readable for PricIo4b0Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io4b0::W`](W) writer structure"]
impl crate::Writable for PricIo4b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO4B0 to value 0"]
impl crate::Resettable for PricIo4b0Spec {}
