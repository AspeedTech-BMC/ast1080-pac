#[doc = "Register `SPIPF270` reader"]
pub type R = crate::R<Spipf270Spec>;
#[doc = "Register `SPIPF270` writer"]
pub type W = crate::W<Spipf270Spec>;
#[doc = "Field `RegionValid14` reader - Region Valid"]
pub type RegionValid14R = crate::BitReader;
#[doc = "Field `RegionValid14` writer - Region Valid"]
pub type RegionValid14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion14` reader - Disable Write to this region"]
pub type DisWrToThisRegion14R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion14` writer - Disable Write to this region"]
pub type DisWrToThisRegion14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion14` reader - Disable Read to this region"]
pub type DisReadToThisRegion14R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion14` writer - Disable Read to this region"]
pub type DisReadToThisRegion14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved30` reader - Reserved"]
pub type Reserved30R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart14` reader - Region Start"]
pub type RegionStart14R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart14` writer - Region Start"]
pub type RegionStart14W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize14` reader - Region Size"]
pub type RegionSize14R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize14` writer - Region Size"]
pub type RegionSize14W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::FieldReader<u16>;
#[doc = "Field `WrProt14` reader - Write Protection"]
pub type WrProt14R = crate::BitReader;
#[doc = "Field `WrProt14` writer - Write Protection"]
pub type WrProt14W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid14(&self) -> RegionValid14R {
        RegionValid14R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region14(&self) -> DisWrToThisRegion14R {
        DisWrToThisRegion14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region14(&self) -> DisReadToThisRegion14R {
        DisReadToThisRegion14R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start14(&self) -> RegionStart14R {
        RegionStart14R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size14(&self) -> RegionSize14R {
        RegionSize14R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot14(&self) -> WrProt14R {
        WrProt14R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid14(&mut self) -> RegionValid14W<Spipf270Spec> {
        RegionValid14W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region14(&mut self) -> DisWrToThisRegion14W<Spipf270Spec> {
        DisWrToThisRegion14W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region14(&mut self) -> DisReadToThisRegion14W<Spipf270Spec> {
        DisReadToThisRegion14W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start14(&mut self) -> RegionStart14W<Spipf270Spec> {
        RegionStart14W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size14(&mut self) -> RegionSize14W<Spipf270Spec> {
        RegionSize14W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot14(&mut self) -> WrProt14W<Spipf270Spec> {
        WrProt14W::new(self, 63)
    }
}
#[doc = "Region 46 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf270::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf270::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf270Spec;
impl crate::RegisterSpec for Spipf270Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf270::R`](R) reader structure"]
impl crate::Readable for Spipf270Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf270::W`](W) writer structure"]
impl crate::Writable for Spipf270Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF270 to value 0"]
impl crate::Resettable for Spipf270Spec {}
