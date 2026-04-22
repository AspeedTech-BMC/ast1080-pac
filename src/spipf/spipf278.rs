#[doc = "Register `SPIPF278` reader"]
pub type R = crate::R<Spipf278Spec>;
#[doc = "Register `SPIPF278` writer"]
pub type W = crate::W<Spipf278Spec>;
#[doc = "Field `RegionValid15` reader - Region Valid"]
pub type RegionValid15R = crate::BitReader;
#[doc = "Field `RegionValid15` writer - Region Valid"]
pub type RegionValid15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion15` reader - Disable Write to this region"]
pub type DisWrToThisRegion15R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion15` writer - Disable Write to this region"]
pub type DisWrToThisRegion15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion15` reader - Disable Read to this region"]
pub type DisReadToThisRegion15R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion15` writer - Disable Read to this region"]
pub type DisReadToThisRegion15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved31` reader - Reserved"]
pub type Reserved31R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart15` reader - Region Start"]
pub type RegionStart15R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart15` writer - Region Start"]
pub type RegionStart15W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize15` reader - Region Size"]
pub type RegionSize15R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize15` writer - Region Size"]
pub type RegionSize15W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::FieldReader<u16>;
#[doc = "Field `WrProt15` reader - Write Protection"]
pub type WrProt15R = crate::BitReader;
#[doc = "Field `WrProt15` writer - Write Protection"]
pub type WrProt15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid15(&self) -> RegionValid15R {
        RegionValid15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region15(&self) -> DisWrToThisRegion15R {
        DisWrToThisRegion15R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region15(&self) -> DisReadToThisRegion15R {
        DisReadToThisRegion15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start15(&self) -> RegionStart15R {
        RegionStart15R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size15(&self) -> RegionSize15R {
        RegionSize15R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot15(&self) -> WrProt15R {
        WrProt15R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid15(&mut self) -> RegionValid15W<Spipf278Spec> {
        RegionValid15W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region15(&mut self) -> DisWrToThisRegion15W<Spipf278Spec> {
        DisWrToThisRegion15W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region15(&mut self) -> DisReadToThisRegion15W<Spipf278Spec> {
        DisReadToThisRegion15W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start15(&mut self) -> RegionStart15W<Spipf278Spec> {
        RegionStart15W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size15(&mut self) -> RegionSize15W<Spipf278Spec> {
        RegionSize15W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot15(&mut self) -> WrProt15W<Spipf278Spec> {
        WrProt15W::new(self, 63)
    }
}
#[doc = "Region 47 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf278::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf278::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf278Spec;
impl crate::RegisterSpec for Spipf278Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf278::R`](R) reader structure"]
impl crate::Readable for Spipf278Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf278::W`](W) writer structure"]
impl crate::Writable for Spipf278Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF278 to value 0"]
impl crate::Resettable for Spipf278Spec {}
