#[doc = "Register `SPIPF128` reader"]
pub type R = crate::R<Spipf128Spec>;
#[doc = "Register `SPIPF128` writer"]
pub type W = crate::W<Spipf128Spec>;
#[doc = "Field `RegionValid5` reader - Region Valid"]
pub type RegionValid5R = crate::BitReader;
#[doc = "Field `RegionValid5` writer - Region Valid"]
pub type RegionValid5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion5` reader - Disable Write to this region"]
pub type DisWrToThisRegion5R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion5` writer - Disable Write to this region"]
pub type DisWrToThisRegion5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion5` reader - Disable Read to this region"]
pub type DisReadToThisRegion5R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion5` writer - Disable Read to this region"]
pub type DisReadToThisRegion5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved21` reader - Reserved"]
pub type Reserved21R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart5` reader - Region Start"]
pub type RegionStart5R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart5` writer - Region Start"]
pub type RegionStart5W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize5` reader - Region Size"]
pub type RegionSize5R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize5` writer - Region Size"]
pub type RegionSize5W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader<u16>;
#[doc = "Field `WrProt5` reader - Write Protection"]
pub type WrProt5R = crate::BitReader;
#[doc = "Field `WrProt5` writer - Write Protection"]
pub type WrProt5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid5(&self) -> RegionValid5R {
        RegionValid5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region5(&self) -> DisWrToThisRegion5R {
        DisWrToThisRegion5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region5(&self) -> DisReadToThisRegion5R {
        DisReadToThisRegion5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start5(&self) -> RegionStart5R {
        RegionStart5R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size5(&self) -> RegionSize5R {
        RegionSize5R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot5(&self) -> WrProt5R {
        WrProt5R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid5(&mut self) -> RegionValid5W<Spipf128Spec> {
        RegionValid5W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region5(&mut self) -> DisWrToThisRegion5W<Spipf128Spec> {
        DisWrToThisRegion5W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region5(&mut self) -> DisReadToThisRegion5W<Spipf128Spec> {
        DisReadToThisRegion5W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start5(&mut self) -> RegionStart5W<Spipf128Spec> {
        RegionStart5W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size5(&mut self) -> RegionSize5W<Spipf128Spec> {
        RegionSize5W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot5(&mut self) -> WrProt5W<Spipf128Spec> {
        WrProt5W::new(self, 63)
    }
}
#[doc = "Region 05 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf128Spec;
impl crate::RegisterSpec for Spipf128Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf128::R`](R) reader structure"]
impl crate::Readable for Spipf128Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf128::W`](W) writer structure"]
impl crate::Writable for Spipf128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF128 to value 0"]
impl crate::Resettable for Spipf128Spec {}
