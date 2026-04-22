#[doc = "Register `SPIPF260` reader"]
pub type R = crate::R<Spipf260Spec>;
#[doc = "Register `SPIPF260` writer"]
pub type W = crate::W<Spipf260Spec>;
#[doc = "Field `RegionValid12` reader - Region Valid"]
pub type RegionValid12R = crate::BitReader;
#[doc = "Field `RegionValid12` writer - Region Valid"]
pub type RegionValid12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion12` reader - Disable Write to this region"]
pub type DisWrToThisRegion12R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion12` writer - Disable Write to this region"]
pub type DisWrToThisRegion12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion12` reader - Disable Read to this region"]
pub type DisReadToThisRegion12R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion12` writer - Disable Read to this region"]
pub type DisReadToThisRegion12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved28` reader - Reserved"]
pub type Reserved28R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart12` reader - Region Start"]
pub type RegionStart12R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart12` writer - Region Start"]
pub type RegionStart12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize12` reader - Region Size"]
pub type RegionSize12R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize12` writer - Region Size"]
pub type RegionSize12W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::FieldReader<u16>;
#[doc = "Field `WrProt12` reader - Write Protection"]
pub type WrProt12R = crate::BitReader;
#[doc = "Field `WrProt12` writer - Write Protection"]
pub type WrProt12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid12(&self) -> RegionValid12R {
        RegionValid12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region12(&self) -> DisWrToThisRegion12R {
        DisWrToThisRegion12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region12(&self) -> DisReadToThisRegion12R {
        DisReadToThisRegion12R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start12(&self) -> RegionStart12R {
        RegionStart12R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size12(&self) -> RegionSize12R {
        RegionSize12R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot12(&self) -> WrProt12R {
        WrProt12R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid12(&mut self) -> RegionValid12W<Spipf260Spec> {
        RegionValid12W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region12(&mut self) -> DisWrToThisRegion12W<Spipf260Spec> {
        DisWrToThisRegion12W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region12(&mut self) -> DisReadToThisRegion12W<Spipf260Spec> {
        DisReadToThisRegion12W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start12(&mut self) -> RegionStart12W<Spipf260Spec> {
        RegionStart12W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size12(&mut self) -> RegionSize12W<Spipf260Spec> {
        RegionSize12W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot12(&mut self) -> WrProt12W<Spipf260Spec> {
        WrProt12W::new(self, 63)
    }
}
#[doc = "Region 44 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf260::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf260::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf260Spec;
impl crate::RegisterSpec for Spipf260Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf260::R`](R) reader structure"]
impl crate::Readable for Spipf260Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf260::W`](W) writer structure"]
impl crate::Writable for Spipf260Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF260 to value 0"]
impl crate::Resettable for Spipf260Spec {}
