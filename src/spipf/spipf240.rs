#[doc = "Register `SPIPF240` reader"]
pub type R = crate::R<Spipf240Spec>;
#[doc = "Register `SPIPF240` writer"]
pub type W = crate::W<Spipf240Spec>;
#[doc = "Field `RegionValid8` reader - Region Valid"]
pub type RegionValid8R = crate::BitReader;
#[doc = "Field `RegionValid8` writer - Region Valid"]
pub type RegionValid8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion8` reader - Disable Write to this region"]
pub type DisWrToThisRegion8R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion8` writer - Disable Write to this region"]
pub type DisWrToThisRegion8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion8` reader - Disable Read to this region"]
pub type DisReadToThisRegion8R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion8` writer - Disable Read to this region"]
pub type DisReadToThisRegion8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved24` reader - Reserved"]
pub type Reserved24R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart8` reader - Region Start"]
pub type RegionStart8R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart8` writer - Region Start"]
pub type RegionStart8W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize8` reader - Region Size"]
pub type RegionSize8R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize8` writer - Region Size"]
pub type RegionSize8W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::FieldReader<u16>;
#[doc = "Field `WrProt8` reader - Write Protection"]
pub type WrProt8R = crate::BitReader;
#[doc = "Field `WrProt8` writer - Write Protection"]
pub type WrProt8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid8(&self) -> RegionValid8R {
        RegionValid8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region8(&self) -> DisWrToThisRegion8R {
        DisWrToThisRegion8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region8(&self) -> DisReadToThisRegion8R {
        DisReadToThisRegion8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start8(&self) -> RegionStart8R {
        RegionStart8R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size8(&self) -> RegionSize8R {
        RegionSize8R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot8(&self) -> WrProt8R {
        WrProt8R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid8(&mut self) -> RegionValid8W<Spipf240Spec> {
        RegionValid8W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region8(&mut self) -> DisWrToThisRegion8W<Spipf240Spec> {
        DisWrToThisRegion8W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region8(&mut self) -> DisReadToThisRegion8W<Spipf240Spec> {
        DisReadToThisRegion8W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start8(&mut self) -> RegionStart8W<Spipf240Spec> {
        RegionStart8W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size8(&mut self) -> RegionSize8W<Spipf240Spec> {
        RegionSize8W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot8(&mut self) -> WrProt8W<Spipf240Spec> {
        WrProt8W::new(self, 63)
    }
}
#[doc = "Region 40 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf240::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf240::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf240Spec;
impl crate::RegisterSpec for Spipf240Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf240::R`](R) reader structure"]
impl crate::Readable for Spipf240Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf240::W`](W) writer structure"]
impl crate::Writable for Spipf240Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF240 to value 0"]
impl crate::Resettable for Spipf240Spec {}
