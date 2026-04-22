#[doc = "Register `SPIPF110` reader"]
pub type R = crate::R<Spipf110Spec>;
#[doc = "Register `SPIPF110` writer"]
pub type W = crate::W<Spipf110Spec>;
#[doc = "Field `RegionValid2` reader - Region Valid"]
pub type RegionValid2R = crate::BitReader;
#[doc = "Field `RegionValid2` writer - Region Valid"]
pub type RegionValid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion2` reader - Disable Write to this region"]
pub type DisWrToThisRegion2R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion2` writer - Disable Write to this region"]
pub type DisWrToThisRegion2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion2` reader - Disable Read to this region"]
pub type DisReadToThisRegion2R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion2` writer - Disable Read to this region"]
pub type DisReadToThisRegion2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved18` reader - Reserved"]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart2` reader - Region Start"]
pub type RegionStart2R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart2` writer - Region Start"]
pub type RegionStart2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize2` reader - Region Size"]
pub type RegionSize2R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize2` writer - Region Size"]
pub type RegionSize2W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `WrProt2` reader - Write Protection"]
pub type WrProt2R = crate::BitReader;
#[doc = "Field `WrProt2` writer - Write Protection"]
pub type WrProt2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid2(&self) -> RegionValid2R {
        RegionValid2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region2(&self) -> DisWrToThisRegion2R {
        DisWrToThisRegion2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region2(&self) -> DisReadToThisRegion2R {
        DisReadToThisRegion2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start2(&self) -> RegionStart2R {
        RegionStart2R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size2(&self) -> RegionSize2R {
        RegionSize2R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot2(&self) -> WrProt2R {
        WrProt2R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid2(&mut self) -> RegionValid2W<Spipf110Spec> {
        RegionValid2W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region2(&mut self) -> DisWrToThisRegion2W<Spipf110Spec> {
        DisWrToThisRegion2W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region2(&mut self) -> DisReadToThisRegion2W<Spipf110Spec> {
        DisReadToThisRegion2W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start2(&mut self) -> RegionStart2W<Spipf110Spec> {
        RegionStart2W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size2(&mut self) -> RegionSize2W<Spipf110Spec> {
        RegionSize2W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot2(&mut self) -> WrProt2W<Spipf110Spec> {
        WrProt2W::new(self, 63)
    }
}
#[doc = "Region 02 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf110Spec;
impl crate::RegisterSpec for Spipf110Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf110::R`](R) reader structure"]
impl crate::Readable for Spipf110Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf110::W`](W) writer structure"]
impl crate::Writable for Spipf110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF110 to value 0"]
impl crate::Resettable for Spipf110Spec {}
