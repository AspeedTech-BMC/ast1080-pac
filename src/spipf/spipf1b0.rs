#[doc = "Register `SPIPF1B0` reader"]
pub type R = crate::R<Spipf1b0Spec>;
#[doc = "Register `SPIPF1B0` writer"]
pub type W = crate::W<Spipf1b0Spec>;
#[doc = "Field `RegionValid6` reader - Region Valid"]
pub type RegionValid6R = crate::BitReader;
#[doc = "Field `RegionValid6` writer - Region Valid"]
pub type RegionValid6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion6` reader - Disable Write to this region"]
pub type DisWrToThisRegion6R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion6` writer - Disable Write to this region"]
pub type DisWrToThisRegion6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion6` reader - Disable Read to this region"]
pub type DisReadToThisRegion6R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion6` writer - Disable Read to this region"]
pub type DisReadToThisRegion6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved22` reader - Reserved"]
pub type Reserved22R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart6` reader - Region Start"]
pub type RegionStart6R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart6` writer - Region Start"]
pub type RegionStart6W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize6` reader - Region Size"]
pub type RegionSize6R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize6` writer - Region Size"]
pub type RegionSize6W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader<u16>;
#[doc = "Field `WrProt6` reader - Write Protection"]
pub type WrProt6R = crate::BitReader;
#[doc = "Field `WrProt6` writer - Write Protection"]
pub type WrProt6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid6(&self) -> RegionValid6R {
        RegionValid6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region6(&self) -> DisWrToThisRegion6R {
        DisWrToThisRegion6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region6(&self) -> DisReadToThisRegion6R {
        DisReadToThisRegion6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start6(&self) -> RegionStart6R {
        RegionStart6R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size6(&self) -> RegionSize6R {
        RegionSize6R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot6(&self) -> WrProt6R {
        WrProt6R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid6(&mut self) -> RegionValid6W<Spipf1b0Spec> {
        RegionValid6W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region6(&mut self) -> DisWrToThisRegion6W<Spipf1b0Spec> {
        DisWrToThisRegion6W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region6(&mut self) -> DisReadToThisRegion6W<Spipf1b0Spec> {
        DisReadToThisRegion6W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start6(&mut self) -> RegionStart6W<Spipf1b0Spec> {
        RegionStart6W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size6(&mut self) -> RegionSize6W<Spipf1b0Spec> {
        RegionSize6W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot6(&mut self) -> WrProt6W<Spipf1b0Spec> {
        WrProt6W::new(self, 63)
    }
}
#[doc = "Region 22 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1b0Spec;
impl crate::RegisterSpec for Spipf1b0Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf1b0::R`](R) reader structure"]
impl crate::Readable for Spipf1b0Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf1b0::W`](W) writer structure"]
impl crate::Writable for Spipf1b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1B0 to value 0"]
impl crate::Resettable for Spipf1b0Spec {}
