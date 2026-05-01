#[doc = "Register `AT094` reader"]
pub type R = crate::R<At094Spec>;
#[doc = "Register `AT094` writer"]
pub type W = crate::W<At094Spec>;
#[doc = "Field `ATGDMST0` reader - AT_GD_MST_0"]
pub type Atgdmst0R = crate::FieldReader;
#[doc = "Field `ATGDMST0` writer - AT_GD_MST_0"]
pub type Atgdmst0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD0` reader - AT_GD_MST_VLD_0"]
pub type Atgdmstvld0R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD0` writer - AT_GD_MST_VLD_0"]
pub type Atgdmstvld0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDMST1` reader - AT_GD_MST_1"]
pub type Atgdmst1R = crate::FieldReader;
#[doc = "Field `ATGDMST1` writer - AT_GD_MST_1"]
pub type Atgdmst1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD1` reader - AT_GD_MST_VLD_1"]
pub type Atgdmstvld1R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD1` writer - AT_GD_MST_VLD_1"]
pub type Atgdmstvld1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDMST2` reader - AT_GD_MST_2"]
pub type Atgdmst2R = crate::FieldReader;
#[doc = "Field `ATGDMST2` writer - AT_GD_MST_2"]
pub type Atgdmst2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD2` reader - AT_GD_MST_VLD_2"]
pub type Atgdmstvld2R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD2` writer - AT_GD_MST_VLD_2"]
pub type Atgdmstvld2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDMST3` reader - AT_GD_MST_3"]
pub type Atgdmst3R = crate::FieldReader;
#[doc = "Field `ATGDMST3` writer - AT_GD_MST_3"]
pub type Atgdmst3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ATGDMSTVLD3` reader - AT_GD_MST_VLD_3"]
pub type Atgdmstvld3R = crate::BitReader;
#[doc = "Field `ATGDMSTVLD3` writer - AT_GD_MST_VLD_3"]
pub type Atgdmstvld3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - AT_GD_MST_0"]
    #[inline(always)]
    pub fn atgdmst0(&self) -> Atgdmst0R {
        Atgdmst0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AT_GD_MST_VLD_0"]
    #[inline(always)]
    pub fn atgdmstvld0(&self) -> Atgdmstvld0R {
        Atgdmstvld0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - AT_GD_MST_1"]
    #[inline(always)]
    pub fn atgdmst1(&self) -> Atgdmst1R {
        Atgdmst1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AT_GD_MST_VLD_1"]
    #[inline(always)]
    pub fn atgdmstvld1(&self) -> Atgdmstvld1R {
        Atgdmstvld1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - AT_GD_MST_2"]
    #[inline(always)]
    pub fn atgdmst2(&self) -> Atgdmst2R {
        Atgdmst2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AT_GD_MST_VLD_2"]
    #[inline(always)]
    pub fn atgdmstvld2(&self) -> Atgdmstvld2R {
        Atgdmstvld2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - AT_GD_MST_3"]
    #[inline(always)]
    pub fn atgdmst3(&self) -> Atgdmst3R {
        Atgdmst3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - AT_GD_MST_VLD_3"]
    #[inline(always)]
    pub fn atgdmstvld3(&self) -> Atgdmstvld3R {
        Atgdmstvld3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - AT_GD_MST_0"]
    #[inline(always)]
    pub fn atgdmst0(&mut self) -> Atgdmst0W<At094Spec> {
        Atgdmst0W::new(self, 0)
    }
    #[doc = "Bit 7 - AT_GD_MST_VLD_0"]
    #[inline(always)]
    pub fn atgdmstvld0(&mut self) -> Atgdmstvld0W<At094Spec> {
        Atgdmstvld0W::new(self, 7)
    }
    #[doc = "Bits 8:13 - AT_GD_MST_1"]
    #[inline(always)]
    pub fn atgdmst1(&mut self) -> Atgdmst1W<At094Spec> {
        Atgdmst1W::new(self, 8)
    }
    #[doc = "Bit 15 - AT_GD_MST_VLD_1"]
    #[inline(always)]
    pub fn atgdmstvld1(&mut self) -> Atgdmstvld1W<At094Spec> {
        Atgdmstvld1W::new(self, 15)
    }
    #[doc = "Bits 16:21 - AT_GD_MST_2"]
    #[inline(always)]
    pub fn atgdmst2(&mut self) -> Atgdmst2W<At094Spec> {
        Atgdmst2W::new(self, 16)
    }
    #[doc = "Bit 23 - AT_GD_MST_VLD_2"]
    #[inline(always)]
    pub fn atgdmstvld2(&mut self) -> Atgdmstvld2W<At094Spec> {
        Atgdmstvld2W::new(self, 23)
    }
    #[doc = "Bits 24:29 - AT_GD_MST_3"]
    #[inline(always)]
    pub fn atgdmst3(&mut self) -> Atgdmst3W<At094Spec> {
        Atgdmst3W::new(self, 24)
    }
    #[doc = "Bit 31 - AT_GD_MST_VLD_3"]
    #[inline(always)]
    pub fn atgdmstvld3(&mut self) -> Atgdmstvld3W<At094Spec> {
        Atgdmstvld3W::new(self, 31)
    }
}
#[doc = "Glitch Detection Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At094Spec;
impl crate::RegisterSpec for At094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at094::R`](R) reader structure"]
impl crate::Readable for At094Spec {}
#[doc = "`write(|w| ..)` method takes [`at094::W`](W) writer structure"]
impl crate::Writable for At094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT094 to value 0"]
impl crate::Resettable for At094Spec {}
