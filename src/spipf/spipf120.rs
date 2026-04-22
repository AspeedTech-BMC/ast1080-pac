#[doc = "Register `SPIPF120` reader"]
pub type R = crate::R<Spipf120Spec>;
#[doc = "Register `SPIPF120` writer"]
pub type W = crate::W<Spipf120Spec>;
#[doc = "Field `RegionValid4` reader - Region Valid"]
pub type RegionValid4R = crate::BitReader;
#[doc = "Field `RegionValid4` writer - Region Valid"]
pub type RegionValid4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion4` reader - Disable Write to this region"]
pub type DisWrToThisRegion4R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion4` writer - Disable Write to this region"]
pub type DisWrToThisRegion4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion4` reader - Disable Read to this region"]
pub type DisReadToThisRegion4R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion4` writer - Disable Read to this region"]
pub type DisReadToThisRegion4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved20` reader - Reserved"]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart4` reader - Region Start"]
pub type RegionStart4R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart4` writer - Region Start"]
pub type RegionStart4W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize4` reader - Region Size"]
pub type RegionSize4R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize4` writer - Region Size"]
pub type RegionSize4W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `WrProt4` reader - Write Protection"]
pub type WrProt4R = crate::BitReader;
#[doc = "Field `WrProt4` writer - Write Protection"]
pub type WrProt4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid4(&self) -> RegionValid4R {
        RegionValid4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region4(&self) -> DisWrToThisRegion4R {
        DisWrToThisRegion4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region4(&self) -> DisReadToThisRegion4R {
        DisReadToThisRegion4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start4(&self) -> RegionStart4R {
        RegionStart4R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size4(&self) -> RegionSize4R {
        RegionSize4R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot4(&self) -> WrProt4R {
        WrProt4R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid4(&mut self) -> RegionValid4W<Spipf120Spec> {
        RegionValid4W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region4(&mut self) -> DisWrToThisRegion4W<Spipf120Spec> {
        DisWrToThisRegion4W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region4(&mut self) -> DisReadToThisRegion4W<Spipf120Spec> {
        DisReadToThisRegion4W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start4(&mut self) -> RegionStart4W<Spipf120Spec> {
        RegionStart4W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size4(&mut self) -> RegionSize4W<Spipf120Spec> {
        RegionSize4W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot4(&mut self) -> WrProt4W<Spipf120Spec> {
        WrProt4W::new(self, 63)
    }
}
#[doc = "Region 04 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf120Spec;
impl crate::RegisterSpec for Spipf120Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf120::R`](R) reader structure"]
impl crate::Readable for Spipf120Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf120::W`](W) writer structure"]
impl crate::Writable for Spipf120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF120 to value 0"]
impl crate::Resettable for Spipf120Spec {}
