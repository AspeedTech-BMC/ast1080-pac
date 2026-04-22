#[doc = "Register `SPIPF280` reader"]
pub type R = crate::R<Spipf280Spec>;
#[doc = "Register `SPIPF280` writer"]
pub type W = crate::W<Spipf280Spec>;
#[doc = "Field `RegionValid` reader - Region Valid"]
pub type RegionValidR = crate::BitReader;
#[doc = "Field `RegionValid` writer - Region Valid"]
pub type RegionValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion` reader - Disable Write to this region"]
pub type DisWrToThisRegionR = crate::BitReader;
#[doc = "Field `DisWrToThisRegion` writer - Disable Write to this region"]
pub type DisWrToThisRegionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion` reader - Disable Read to this region"]
pub type DisReadToThisRegionR = crate::BitReader;
#[doc = "Field `DisReadToThisRegion` writer - Disable Read to this region"]
pub type DisReadToThisRegionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved16` reader - Reserved"]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart` reader - Region Start"]
pub type RegionStartR = crate::FieldReader<u32>;
#[doc = "Field `RegionStart` writer - Region Start"]
pub type RegionStartW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize` reader - Region Size"]
pub type RegionSizeR = crate::FieldReader<u32>;
#[doc = "Field `RegionSize` writer - Region Size"]
pub type RegionSizeW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `WrProt` reader - Write Protection"]
pub type WrProtR = crate::BitReader;
#[doc = "Field `WrProt` writer - Write Protection"]
pub type WrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid(&self) -> RegionValidR {
        RegionValidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region(&self) -> DisWrToThisRegionR {
        DisWrToThisRegionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region(&self) -> DisReadToThisRegionR {
        DisReadToThisRegionR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start(&self) -> RegionStartR {
        RegionStartR::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size(&self) -> RegionSizeR {
        RegionSizeR::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot(&self) -> WrProtR {
        WrProtR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid(&mut self) -> RegionValidW<Spipf280Spec> {
        RegionValidW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region(&mut self) -> DisWrToThisRegionW<Spipf280Spec> {
        DisWrToThisRegionW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region(&mut self) -> DisReadToThisRegionW<Spipf280Spec> {
        DisReadToThisRegionW::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start(&mut self) -> RegionStartW<Spipf280Spec> {
        RegionStartW::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size(&mut self) -> RegionSizeW<Spipf280Spec> {
        RegionSizeW::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot(&mut self) -> WrProtW<Spipf280Spec> {
        WrProtW::new(self, 63)
    }
}
#[doc = "Region 48 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf280::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf280::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf280Spec;
impl crate::RegisterSpec for Spipf280Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf280::R`](R) reader structure"]
impl crate::Readable for Spipf280Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf280::W`](W) writer structure"]
impl crate::Writable for Spipf280Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF280 to value 0"]
impl crate::Resettable for Spipf280Spec {}
