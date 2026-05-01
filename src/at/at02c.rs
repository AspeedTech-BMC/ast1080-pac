#[doc = "Register `AT02C` reader"]
pub type R = crate::R<At02cSpec>;
#[doc = "Register `AT02C` writer"]
pub type W = crate::W<At02cSpec>;
#[doc = "Field `ATCAMMST0` reader - AT_CAM_MST_0"]
pub type Atcammst0R = crate::FieldReader;
#[doc = "Field `ATCAMMST0` writer - AT_CAM_MST_0"]
pub type Atcammst0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD0` reader - AT_CAM_MST_VLD_0"]
pub type Atcammstvld0R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD0` writer - AT_CAM_MST_VLD_0"]
pub type Atcammstvld0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMMST1` reader - AT_CAM_MST_1"]
pub type Atcammst1R = crate::FieldReader;
#[doc = "Field `ATCAMMST1` writer - AT_CAM_MST_1"]
pub type Atcammst1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD1` reader - AT_CAM_MST_VLD_1"]
pub type Atcammstvld1R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD1` writer - AT_CAM_MST_VLD_1"]
pub type Atcammstvld1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMMST2` reader - AT_CAM_MST_2"]
pub type Atcammst2R = crate::FieldReader;
#[doc = "Field `ATCAMMST2` writer - AT_CAM_MST_2"]
pub type Atcammst2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD2` reader - AT_CAM_MST_VLD_2"]
pub type Atcammstvld2R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD2` writer - AT_CAM_MST_VLD_2"]
pub type Atcammstvld2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMMST3` reader - AT_CAM_MST_3"]
pub type Atcammst3R = crate::FieldReader;
#[doc = "Field `ATCAMMST3` writer - AT_CAM_MST_3"]
pub type Atcammst3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ATCAMMSTVLD3` reader - AT_CAM_MST_VLD_3"]
pub type Atcammstvld3R = crate::BitReader;
#[doc = "Field `ATCAMMSTVLD3` writer - AT_CAM_MST_VLD_3"]
pub type Atcammstvld3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - AT_CAM_MST_0"]
    #[inline(always)]
    pub fn atcammst0(&self) -> Atcammst0R {
        Atcammst0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AT_CAM_MST_VLD_0"]
    #[inline(always)]
    pub fn atcammstvld0(&self) -> Atcammstvld0R {
        Atcammstvld0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - AT_CAM_MST_1"]
    #[inline(always)]
    pub fn atcammst1(&self) -> Atcammst1R {
        Atcammst1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AT_CAM_MST_VLD_1"]
    #[inline(always)]
    pub fn atcammstvld1(&self) -> Atcammstvld1R {
        Atcammstvld1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - AT_CAM_MST_2"]
    #[inline(always)]
    pub fn atcammst2(&self) -> Atcammst2R {
        Atcammst2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AT_CAM_MST_VLD_2"]
    #[inline(always)]
    pub fn atcammstvld2(&self) -> Atcammstvld2R {
        Atcammstvld2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - AT_CAM_MST_3"]
    #[inline(always)]
    pub fn atcammst3(&self) -> Atcammst3R {
        Atcammst3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - AT_CAM_MST_VLD_3"]
    #[inline(always)]
    pub fn atcammstvld3(&self) -> Atcammstvld3R {
        Atcammstvld3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - AT_CAM_MST_0"]
    #[inline(always)]
    pub fn atcammst0(&mut self) -> Atcammst0W<At02cSpec> {
        Atcammst0W::new(self, 0)
    }
    #[doc = "Bit 7 - AT_CAM_MST_VLD_0"]
    #[inline(always)]
    pub fn atcammstvld0(&mut self) -> Atcammstvld0W<At02cSpec> {
        Atcammstvld0W::new(self, 7)
    }
    #[doc = "Bits 8:13 - AT_CAM_MST_1"]
    #[inline(always)]
    pub fn atcammst1(&mut self) -> Atcammst1W<At02cSpec> {
        Atcammst1W::new(self, 8)
    }
    #[doc = "Bit 15 - AT_CAM_MST_VLD_1"]
    #[inline(always)]
    pub fn atcammstvld1(&mut self) -> Atcammstvld1W<At02cSpec> {
        Atcammstvld1W::new(self, 15)
    }
    #[doc = "Bits 16:21 - AT_CAM_MST_2"]
    #[inline(always)]
    pub fn atcammst2(&mut self) -> Atcammst2W<At02cSpec> {
        Atcammst2W::new(self, 16)
    }
    #[doc = "Bit 23 - AT_CAM_MST_VLD_2"]
    #[inline(always)]
    pub fn atcammstvld2(&mut self) -> Atcammstvld2W<At02cSpec> {
        Atcammstvld2W::new(self, 23)
    }
    #[doc = "Bits 24:29 - AT_CAM_MST_3"]
    #[inline(always)]
    pub fn atcammst3(&mut self) -> Atcammst3W<At02cSpec> {
        Atcammst3W::new(self, 24)
    }
    #[doc = "Bit 31 - AT_CAM_MST_VLD_3"]
    #[inline(always)]
    pub fn atcammstvld3(&mut self) -> Atcammstvld3W<At02cSpec> {
        Atcammstvld3W::new(self, 31)
    }
}
#[doc = "Clock Attack Monitor Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At02cSpec;
impl crate::RegisterSpec for At02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at02c::R`](R) reader structure"]
impl crate::Readable for At02cSpec {}
#[doc = "`write(|w| ..)` method takes [`at02c::W`](W) writer structure"]
impl crate::Writable for At02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT02C to value 0"]
impl crate::Resettable for At02cSpec {}
