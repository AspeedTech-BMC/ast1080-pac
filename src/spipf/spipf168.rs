#[doc = "Register `SPIPF168` reader"]
pub type R = crate::R<Spipf168Spec>;
#[doc = "Register `SPIPF168` writer"]
pub type W = crate::W<Spipf168Spec>;
#[doc = "Field `RegionValid13` reader - Region Valid"]
pub type RegionValid13R = crate::BitReader;
#[doc = "Field `RegionValid13` writer - Region Valid"]
pub type RegionValid13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion13` reader - Disable Write to this region"]
pub type DisWrToThisRegion13R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion13` writer - Disable Write to this region"]
pub type DisWrToThisRegion13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion13` reader - Disable Read to this region"]
pub type DisReadToThisRegion13R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion13` writer - Disable Read to this region"]
pub type DisReadToThisRegion13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved29` reader - Reserved"]
pub type Reserved29R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart13` reader - Region Start"]
pub type RegionStart13R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart13` writer - Region Start"]
pub type RegionStart13W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize13` reader - Region Size"]
pub type RegionSize13R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize13` writer - Region Size"]
pub type RegionSize13W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::FieldReader<u16>;
#[doc = "Field `WrProt13` reader - Write Protection"]
pub type WrProt13R = crate::BitReader;
#[doc = "Field `WrProt13` writer - Write Protection"]
pub type WrProt13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid13(&self) -> RegionValid13R {
        RegionValid13R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region13(&self) -> DisWrToThisRegion13R {
        DisWrToThisRegion13R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region13(&self) -> DisReadToThisRegion13R {
        DisReadToThisRegion13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start13(&self) -> RegionStart13R {
        RegionStart13R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size13(&self) -> RegionSize13R {
        RegionSize13R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot13(&self) -> WrProt13R {
        WrProt13R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid13(&mut self) -> RegionValid13W<Spipf168Spec> {
        RegionValid13W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region13(&mut self) -> DisWrToThisRegion13W<Spipf168Spec> {
        DisWrToThisRegion13W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region13(&mut self) -> DisReadToThisRegion13W<Spipf168Spec> {
        DisReadToThisRegion13W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start13(&mut self) -> RegionStart13W<Spipf168Spec> {
        RegionStart13W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size13(&mut self) -> RegionSize13W<Spipf168Spec> {
        RegionSize13W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot13(&mut self) -> WrProt13W<Spipf168Spec> {
        WrProt13W::new(self, 63)
    }
}
#[doc = "Region 13 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf168Spec;
impl crate::RegisterSpec for Spipf168Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf168::R`](R) reader structure"]
impl crate::Readable for Spipf168Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf168::W`](W) writer structure"]
impl crate::Writable for Spipf168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF168 to value 0"]
impl crate::Resettable for Spipf168Spec {}
