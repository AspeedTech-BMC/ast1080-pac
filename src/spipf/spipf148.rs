#[doc = "Register `SPIPF148` reader"]
pub type R = crate::R<Spipf148Spec>;
#[doc = "Register `SPIPF148` writer"]
pub type W = crate::W<Spipf148Spec>;
#[doc = "Field `RegionValid9` reader - Region Valid"]
pub type RegionValid9R = crate::BitReader;
#[doc = "Field `RegionValid9` writer - Region Valid"]
pub type RegionValid9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion9` reader - Disable Write to this region"]
pub type DisWrToThisRegion9R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion9` writer - Disable Write to this region"]
pub type DisWrToThisRegion9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion9` reader - Disable Read to this region"]
pub type DisReadToThisRegion9R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion9` writer - Disable Read to this region"]
pub type DisReadToThisRegion9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved25` reader - Reserved"]
pub type Reserved25R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart9` reader - Region Start"]
pub type RegionStart9R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart9` writer - Region Start"]
pub type RegionStart9W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize9` reader - Region Size"]
pub type RegionSize9R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize9` writer - Region Size"]
pub type RegionSize9W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::FieldReader<u16>;
#[doc = "Field `WrProt9` reader - Write Protection"]
pub type WrProt9R = crate::BitReader;
#[doc = "Field `WrProt9` writer - Write Protection"]
pub type WrProt9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid9(&self) -> RegionValid9R {
        RegionValid9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region9(&self) -> DisWrToThisRegion9R {
        DisWrToThisRegion9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region9(&self) -> DisReadToThisRegion9R {
        DisReadToThisRegion9R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start9(&self) -> RegionStart9R {
        RegionStart9R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size9(&self) -> RegionSize9R {
        RegionSize9R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot9(&self) -> WrProt9R {
        WrProt9R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid9(&mut self) -> RegionValid9W<Spipf148Spec> {
        RegionValid9W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region9(&mut self) -> DisWrToThisRegion9W<Spipf148Spec> {
        DisWrToThisRegion9W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region9(&mut self) -> DisReadToThisRegion9W<Spipf148Spec> {
        DisReadToThisRegion9W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start9(&mut self) -> RegionStart9W<Spipf148Spec> {
        RegionStart9W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size9(&mut self) -> RegionSize9W<Spipf148Spec> {
        RegionSize9W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot9(&mut self) -> WrProt9W<Spipf148Spec> {
        WrProt9W::new(self, 63)
    }
}
#[doc = "Region 09 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf148Spec;
impl crate::RegisterSpec for Spipf148Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf148::R`](R) reader structure"]
impl crate::Readable for Spipf148Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf148::W`](W) writer structure"]
impl crate::Writable for Spipf148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF148 to value 0"]
impl crate::Resettable for Spipf148Spec {}
