#[doc = "Register `SPIPF2D0` reader"]
pub type R = crate::R<Spipf2d0Spec>;
#[doc = "Register `SPIPF2D0` writer"]
pub type W = crate::W<Spipf2d0Spec>;
#[doc = "Field `RegionValid10` reader - Region Valid"]
pub type RegionValid10R = crate::BitReader;
#[doc = "Field `RegionValid10` writer - Region Valid"]
pub type RegionValid10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisWrToThisRegion10` reader - Disable Write to this region"]
pub type DisWrToThisRegion10R = crate::BitReader;
#[doc = "Field `DisWrToThisRegion10` writer - Disable Write to this region"]
pub type DisWrToThisRegion10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisReadToThisRegion10` reader - Disable Read to this region"]
pub type DisReadToThisRegion10R = crate::BitReader;
#[doc = "Field `DisReadToThisRegion10` writer - Disable Read to this region"]
pub type DisReadToThisRegion10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved26` reader - Reserved"]
pub type Reserved26R = crate::FieldReader<u16>;
#[doc = "Field `RegionStart10` reader - Region Start"]
pub type RegionStart10R = crate::FieldReader<u32>;
#[doc = "Field `RegionStart10` writer - Region Start"]
pub type RegionStart10W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `RegionSize10` reader - Region Size"]
pub type RegionSize10R = crate::FieldReader<u32>;
#[doc = "Field `RegionSize10` writer - Region Size"]
pub type RegionSize10W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::FieldReader<u16>;
#[doc = "Field `WrProt10` reader - Write Protection"]
pub type WrProt10R = crate::BitReader;
#[doc = "Field `WrProt10` writer - Write Protection"]
pub type WrProt10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid10(&self) -> RegionValid10R {
        RegionValid10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region10(&self) -> DisWrToThisRegion10R {
        DisWrToThisRegion10R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region10(&self) -> DisReadToThisRegion10R {
        DisReadToThisRegion10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:11 - Reserved"]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start10(&self) -> RegionStart10R {
        RegionStart10R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size10(&self) -> RegionSize10R {
        RegionSize10R::new(((self.bits >> 32) & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 53:62 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 53) & 0x03ff) as u16)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot10(&self) -> WrProt10R {
        WrProt10R::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region Valid"]
    #[inline(always)]
    pub fn region_valid10(&mut self) -> RegionValid10W<Spipf2d0Spec> {
        RegionValid10W::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Write to this region"]
    #[inline(always)]
    pub fn dis_wr_to_this_region10(&mut self) -> DisWrToThisRegion10W<Spipf2d0Spec> {
        DisWrToThisRegion10W::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Read to this region"]
    #[inline(always)]
    pub fn dis_read_to_this_region10(&mut self) -> DisReadToThisRegion10W<Spipf2d0Spec> {
        DisReadToThisRegion10W::new(self, 2)
    }
    #[doc = "Bits 12:31 - Region Start"]
    #[inline(always)]
    pub fn region_start10(&mut self) -> RegionStart10W<Spipf2d0Spec> {
        RegionStart10W::new(self, 12)
    }
    #[doc = "Bits 32:52 - Region Size"]
    #[inline(always)]
    pub fn region_size10(&mut self) -> RegionSize10W<Spipf2d0Spec> {
        RegionSize10W::new(self, 32)
    }
    #[doc = "Bit 63 - Write Protection"]
    #[inline(always)]
    pub fn wr_prot10(&mut self) -> WrProt10W<Spipf2d0Spec> {
        WrProt10W::new(self, 63)
    }
}
#[doc = "Region 58 setting\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2d0Spec;
impl crate::RegisterSpec for Spipf2d0Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`spipf2d0::R`](R) reader structure"]
impl crate::Readable for Spipf2d0Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf2d0::W`](W) writer structure"]
impl crate::Writable for Spipf2d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2D0 to value 0"]
impl crate::Resettable for Spipf2d0Spec {}
