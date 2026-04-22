#[doc = "Register `SPIPF288` reader"]
pub type R = crate::R<Spipf288Spec>;
#[doc = "Register `SPIPF288` writer"]
pub type W = crate::W<Spipf288Spec>;
#[doc = "Field `RegionValid1` reader - Region Valid"]
pub type RegionValid1R = crate::BitReader;
#[doc = "Field `RegionValid1` writer - Region Valid"]
pub type RegionValid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion1` reader - Disable Write to this region"]
pub type DisWrToThisRegion1R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion1` writer - Disable Write to this region"]
pub type DisWrToThisRegion1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion1` reader - Disable Read to this region"]
pub type DisReadToThisRegion1R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion1` writer - Disable Read to this region"]
pub type DisReadToThisRegion1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved17` reader - Reserved"]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart1` reader - Region Start"]
pub type RegionStart1R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart1` writer - Region Start"]
pub type RegionStart1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize1` reader - Region Size"]
pub type RegionSize1R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize1` writer - Region Size"]
pub type RegionSize1W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `WrProt1` reader - Write Protection"]
pub type WrProt1R = crate::BitReader;
#[doc = "Field `WrProt1` writer - Write Protection"]
pub type WrProt1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid1(&self) -> RegionValid1R {
        RegionValid1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region1(&self) -> DisWrToThisRegion1R {
        DisWrToThisRegion1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region1(&self) -> DisReadToThisRegion1R {
        DisReadToThisRegion1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start1(&self) -> RegionStart1R {
        RegionStart1R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size1(&self) -> RegionSize1R {
        RegionSize1R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot1(&self) -> WrProt1R {
        WrProt1R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid1(&mut self) -> RegionValid1W<Spipf288Spec> {
        RegionValid1W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region1(&mut self) -> DisWrToThisRegion1W<Spipf288Spec> {
        DisWrToThisRegion1W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region1(&mut self) -> DisReadToThisRegion1W<Spipf288Spec> {
        DisReadToThisRegion1W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start1(&mut self) -> RegionStart1W<Spipf288Spec> {
        RegionStart1W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size1(&mut self) -> RegionSize1W<Spipf288Spec> {
        RegionSize1W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot1(&mut self) -> WrProt1W<Spipf288Spec> {
        WrProt1W::new(self, 63)
    }
}
#[doc = "Region 49 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf288::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf288::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf288Spec;
impl crate::RegisterSpec for Spipf288Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf288::R`](R) reader structure"]
impl crate::Readable for Spipf288Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf288::W`](W) writer structure"]
impl crate::Writable for Spipf288Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF288 to value 0"]
impl crate::Resettable for Spipf288Spec {}
