#[doc = "Register `SPIPF2B8` reader"]
pub type R = crate::R<Spipf2b8Spec>;
#[doc = "Register `SPIPF2B8` writer"]
pub type W = crate::W<Spipf2b8Spec>;
#[doc = "Field `RegionValid7` reader - Region Valid"]
pub type RegionValid7R = crate::BitReader;
#[doc = "Field `RegionValid7` writer - Region Valid"]
pub type RegionValid7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion7` reader - Disable Write to this region"]
pub type DisWrToThisRegion7R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion7` writer - Disable Write to this region"]
pub type DisWrToThisRegion7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion7` reader - Disable Read to this region"]
pub type DisReadToThisRegion7R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion7` writer - Disable Read to this region"]
pub type DisReadToThisRegion7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved23` reader - Reserved"]
pub type Reserved23R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart7` reader - Region Start"]
pub type RegionStart7R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart7` writer - Region Start"]
pub type RegionStart7W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize7` reader - Region Size"]
pub type RegionSize7R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize7` writer - Region Size"]
pub type RegionSize7W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::FieldReader<u16>;
#[doc = "Field `WrProt7` reader - Write Protection"]
pub type WrProt7R = crate::BitReader;
#[doc = "Field `WrProt7` writer - Write Protection"]
pub type WrProt7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid7(&self) -> RegionValid7R {
        RegionValid7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region7(&self) -> DisWrToThisRegion7R {
        DisWrToThisRegion7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region7(&self) -> DisReadToThisRegion7R {
        DisReadToThisRegion7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start7(&self) -> RegionStart7R {
        RegionStart7R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size7(&self) -> RegionSize7R {
        RegionSize7R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot7(&self) -> WrProt7R {
        WrProt7R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid7(&mut self) -> RegionValid7W<Spipf2b8Spec> {
        RegionValid7W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region7(&mut self) -> DisWrToThisRegion7W<Spipf2b8Spec> {
        DisWrToThisRegion7W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region7(&mut self) -> DisReadToThisRegion7W<Spipf2b8Spec> {
        DisReadToThisRegion7W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start7(&mut self) -> RegionStart7W<Spipf2b8Spec> {
        RegionStart7W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size7(&mut self) -> RegionSize7W<Spipf2b8Spec> {
        RegionSize7W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot7(&mut self) -> WrProt7W<Spipf2b8Spec> {
        WrProt7W::new(self, 63)
    }
}
#[doc = "Region 55 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2b8Spec;
impl crate::RegisterSpec for Spipf2b8Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf2b8::R`](R) reader structure"]
impl crate::Readable for Spipf2b8Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf2b8::W`](W) writer structure"]
impl crate::Writable for Spipf2b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2B8 to value 0"]
impl crate::Resettable for Spipf2b8Spec {}
