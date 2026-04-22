#[doc = "Register `SPIPF118` reader"]
pub type R = crate::R<Spipf118Spec>;
#[doc = "Register `SPIPF118` writer"]
pub type W = crate::W<Spipf118Spec>;
#[doc = "Field `RegionValid3` reader - Region Valid"]
pub type RegionValid3R = crate::BitReader;
#[doc = "Field `RegionValid3` writer - Region Valid"]
pub type RegionValid3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion3` reader - Disable Write to this region"]
pub type DisWrToThisRegion3R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion3` writer - Disable Write to this region"]
pub type DisWrToThisRegion3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion3` reader - Disable Read to this region"]
pub type DisReadToThisRegion3R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion3` writer - Disable Read to this region"]
pub type DisReadToThisRegion3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved19` reader - Reserved"]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart3` reader - Region Start"]
pub type RegionStart3R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart3` writer - Region Start"]
pub type RegionStart3W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize3` reader - Region Size"]
pub type RegionSize3R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize3` writer - Region Size"]
pub type RegionSize3W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `WrProt3` reader - Write Protection"]
pub type WrProt3R = crate::BitReader;
#[doc = "Field `WrProt3` writer - Write Protection"]
pub type WrProt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid3(&self) -> RegionValid3R {
        RegionValid3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region3(&self) -> DisWrToThisRegion3R {
        DisWrToThisRegion3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region3(&self) -> DisReadToThisRegion3R {
        DisReadToThisRegion3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start3(&self) -> RegionStart3R {
        RegionStart3R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size3(&self) -> RegionSize3R {
        RegionSize3R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot3(&self) -> WrProt3R {
        WrProt3R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid3(&mut self) -> RegionValid3W<Spipf118Spec> {
        RegionValid3W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region3(&mut self) -> DisWrToThisRegion3W<Spipf118Spec> {
        DisWrToThisRegion3W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region3(&mut self) -> DisReadToThisRegion3W<Spipf118Spec> {
        DisReadToThisRegion3W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start3(&mut self) -> RegionStart3W<Spipf118Spec> {
        RegionStart3W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size3(&mut self) -> RegionSize3W<Spipf118Spec> {
        RegionSize3W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot3(&mut self) -> WrProt3W<Spipf118Spec> {
        WrProt3W::new(self, 63)
    }
}
#[doc = "Region 03 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf118Spec;
impl crate::RegisterSpec for Spipf118Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf118::R`](R) reader structure"]
impl crate::Readable for Spipf118Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf118::W`](W) writer structure"]
impl crate::Writable for Spipf118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF118 to value 0"]
impl crate::Resettable for Spipf118Spec {}
