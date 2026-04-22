#[doc = "Register `SPIPF158` reader"]
pub type R = crate::R<Spipf158Spec>;
#[doc = "Register `SPIPF158` writer"]
pub type W = crate::W<Spipf158Spec>;
#[doc = "Field `RegionValid11` reader - Region Valid"]
pub type RegionValid11R = crate::BitReader;
#[doc = "Field `RegionValid11` writer - Region Valid"]
pub type RegionValid11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion11` reader - Disable Write to this region"]
pub type DisWrToThisRegion11R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion11` writer - Disable Write to this region"]
pub type DisWrToThisRegion11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion11` reader - Disable Read to this region"]
pub type DisReadToThisRegion11R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion11` writer - Disable Read to this region"]
pub type DisReadToThisRegion11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved27` reader - Reserved"]
pub type Reserved27R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart11` reader - Region Start"]
pub type RegionStart11R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart11` writer - Region Start"]
pub type RegionStart11W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize11` reader - Region Size"]
pub type RegionSize11R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize11` writer - Region Size"]
pub type RegionSize11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::FieldReader<u16>;
#[doc = "Field `WrProt11` reader - Write Protection"]
pub type WrProt11R = crate::BitReader;
#[doc = "Field `WrProt11` writer - Write Protection"]
pub type WrProt11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid11(&self) -> RegionValid11R {
        RegionValid11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region11(&self) -> DisWrToThisRegion11R {
        DisWrToThisRegion11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region11(&self) -> DisReadToThisRegion11R {
        DisReadToThisRegion11R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start11(&self) -> RegionStart11R {
        RegionStart11R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size11(&self) -> RegionSize11R {
        RegionSize11R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot11(&self) -> WrProt11R {
        WrProt11R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid11(&mut self) -> RegionValid11W<Spipf158Spec> {
        RegionValid11W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region11(&mut self) -> DisWrToThisRegion11W<Spipf158Spec> {
        DisWrToThisRegion11W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region11(&mut self) -> DisReadToThisRegion11W<Spipf158Spec> {
        DisReadToThisRegion11W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start11(&mut self) -> RegionStart11W<Spipf158Spec> {
        RegionStart11W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size11(&mut self) -> RegionSize11W<Spipf158Spec> {
        RegionSize11W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot11(&mut self) -> WrProt11W<Spipf158Spec> {
        WrProt11W::new(self, 63)
    }
}
#[doc = "Region 11 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf158Spec;
impl crate::RegisterSpec for Spipf158Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf158::R`](R) reader structure"]
impl crate::Readable for Spipf158Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf158::W`](W) writer structure"]
impl crate::Writable for Spipf158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF158 to value 0"]
impl crate::Resettable for Spipf158Spec {}
