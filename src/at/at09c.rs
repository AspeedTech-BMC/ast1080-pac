#[doc = "Register `AT09C` reader"]
pub type R = crate::R<At09cSpec>;
#[doc = "Register `AT09C` writer"]
pub type W = crate::W<At09cSpec>;
#[doc = "Field `ATTDMST0` reader - AT_TD_MST_0"]
pub type Attdmst0R = crate::FieldReader;
#[doc = "Field `ATTDMST0` writer - AT_TD_MST_0"]
pub type Attdmst0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD0` reader - AT_TD_MST_VLD_0"]
pub type Attdmstvld0R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD0` writer - AT_TD_MST_VLD_0"]
pub type Attdmstvld0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDMST1` reader - AT_TD_MST_1"]
pub type Attdmst1R = crate::FieldReader;
#[doc = "Field `ATTDMST1` writer - AT_TD_MST_1"]
pub type Attdmst1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD1` reader - AT_TD_MST_VLD_1"]
pub type Attdmstvld1R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD1` writer - AT_TD_MST_VLD_1"]
pub type Attdmstvld1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDMST2` reader - AT_TD_MST_2"]
pub type Attdmst2R = crate::FieldReader;
#[doc = "Field `ATTDMST2` writer - AT_TD_MST_2"]
pub type Attdmst2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD2` reader - AT_TD_MST_VLD_2"]
pub type Attdmstvld2R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD2` writer - AT_TD_MST_VLD_2"]
pub type Attdmstvld2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDMST3` reader - AT_TD_MST_3"]
pub type Attdmst3R = crate::FieldReader;
#[doc = "Field `ATTDMST3` writer - AT_TD_MST_3"]
pub type Attdmst3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ATTDMSTVLD3` reader - AT_TD_MST_VLD_3"]
pub type Attdmstvld3R = crate::BitReader;
#[doc = "Field `ATTDMSTVLD3` writer - AT_TD_MST_VLD_3"]
pub type Attdmstvld3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - AT_TD_MST_0"]
    #[inline(always)]
    pub fn attdmst0(&self) -> Attdmst0R {
        Attdmst0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AT_TD_MST_VLD_0"]
    #[inline(always)]
    pub fn attdmstvld0(&self) -> Attdmstvld0R {
        Attdmstvld0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - AT_TD_MST_1"]
    #[inline(always)]
    pub fn attdmst1(&self) -> Attdmst1R {
        Attdmst1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AT_TD_MST_VLD_1"]
    #[inline(always)]
    pub fn attdmstvld1(&self) -> Attdmstvld1R {
        Attdmstvld1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - AT_TD_MST_2"]
    #[inline(always)]
    pub fn attdmst2(&self) -> Attdmst2R {
        Attdmst2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AT_TD_MST_VLD_2"]
    #[inline(always)]
    pub fn attdmstvld2(&self) -> Attdmstvld2R {
        Attdmstvld2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - AT_TD_MST_3"]
    #[inline(always)]
    pub fn attdmst3(&self) -> Attdmst3R {
        Attdmst3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - AT_TD_MST_VLD_3"]
    #[inline(always)]
    pub fn attdmstvld3(&self) -> Attdmstvld3R {
        Attdmstvld3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - AT_TD_MST_0"]
    #[inline(always)]
    pub fn attdmst0(&mut self) -> Attdmst0W<At09cSpec> {
        Attdmst0W::new(self, 0)
    }
    #[doc = "Bit 7 - AT_TD_MST_VLD_0"]
    #[inline(always)]
    pub fn attdmstvld0(&mut self) -> Attdmstvld0W<At09cSpec> {
        Attdmstvld0W::new(self, 7)
    }
    #[doc = "Bits 8:13 - AT_TD_MST_1"]
    #[inline(always)]
    pub fn attdmst1(&mut self) -> Attdmst1W<At09cSpec> {
        Attdmst1W::new(self, 8)
    }
    #[doc = "Bit 15 - AT_TD_MST_VLD_1"]
    #[inline(always)]
    pub fn attdmstvld1(&mut self) -> Attdmstvld1W<At09cSpec> {
        Attdmstvld1W::new(self, 15)
    }
    #[doc = "Bits 16:21 - AT_TD_MST_2"]
    #[inline(always)]
    pub fn attdmst2(&mut self) -> Attdmst2W<At09cSpec> {
        Attdmst2W::new(self, 16)
    }
    #[doc = "Bit 23 - AT_TD_MST_VLD_2"]
    #[inline(always)]
    pub fn attdmstvld2(&mut self) -> Attdmstvld2W<At09cSpec> {
        Attdmstvld2W::new(self, 23)
    }
    #[doc = "Bits 24:29 - AT_TD_MST_3"]
    #[inline(always)]
    pub fn attdmst3(&mut self) -> Attdmst3W<At09cSpec> {
        Attdmst3W::new(self, 24)
    }
    #[doc = "Bit 31 - AT_TD_MST_VLD_3"]
    #[inline(always)]
    pub fn attdmstvld3(&mut self) -> Attdmstvld3W<At09cSpec> {
        Attdmstvld3W::new(self, 31)
    }
}
#[doc = "TSENSE Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at09c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at09c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At09cSpec;
impl crate::RegisterSpec for At09cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at09c::R`](R) reader structure"]
impl crate::Readable for At09cSpec {}
#[doc = "`write(|w| ..)` method takes [`at09c::W`](W) writer structure"]
impl crate::Writable for At09cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT09C to value 0"]
impl crate::Resettable for At09cSpec {}
