#[doc = "Register `AT098` reader"]
pub type R = crate::R<At098Spec>;
#[doc = "Register `AT098` writer"]
pub type W = crate::W<At098Spec>;
#[doc = "Field `ATIRMST0` reader - AT_IR_MST_0"]
pub type Atirmst0R = crate::FieldReader;
#[doc = "Field `ATIRMST0` writer - AT_IR_MST_0"]
pub type Atirmst0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD0` reader - AT_IR_MST_VLD_0"]
pub type Atirmstvld0R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD0` writer - AT_IR_MST_VLD_0"]
pub type Atirmstvld0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIRMST1` reader - AT_IR_MST_1"]
pub type Atirmst1R = crate::FieldReader;
#[doc = "Field `ATIRMST1` writer - AT_IR_MST_1"]
pub type Atirmst1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD1` reader - AT_IR_MST_VLD_1"]
pub type Atirmstvld1R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD1` writer - AT_IR_MST_VLD_1"]
pub type Atirmstvld1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIRMST2` reader - AT_IR_MST_2"]
pub type Atirmst2R = crate::FieldReader;
#[doc = "Field `ATIRMST2` writer - AT_IR_MST_2"]
pub type Atirmst2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD2` reader - AT_IR_MST_VLD_2"]
pub type Atirmstvld2R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD2` writer - AT_IR_MST_VLD_2"]
pub type Atirmstvld2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATIRMST3` reader - AT_IR_MST_3"]
pub type Atirmst3R = crate::FieldReader;
#[doc = "Field `ATIRMST3` writer - AT_IR_MST_3"]
pub type Atirmst3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ATIRMSTVLD3` reader - AT_IR_MST_VLD_3"]
pub type Atirmstvld3R = crate::BitReader;
#[doc = "Field `ATIRMSTVLD3` writer - AT_IR_MST_VLD_3"]
pub type Atirmstvld3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - AT_IR_MST_0"]
    #[inline(always)]
    pub fn atirmst0(&self) -> Atirmst0R {
        Atirmst0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AT_IR_MST_VLD_0"]
    #[inline(always)]
    pub fn atirmstvld0(&self) -> Atirmstvld0R {
        Atirmstvld0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - AT_IR_MST_1"]
    #[inline(always)]
    pub fn atirmst1(&self) -> Atirmst1R {
        Atirmst1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AT_IR_MST_VLD_1"]
    #[inline(always)]
    pub fn atirmstvld1(&self) -> Atirmstvld1R {
        Atirmstvld1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - AT_IR_MST_2"]
    #[inline(always)]
    pub fn atirmst2(&self) -> Atirmst2R {
        Atirmst2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AT_IR_MST_VLD_2"]
    #[inline(always)]
    pub fn atirmstvld2(&self) -> Atirmstvld2R {
        Atirmstvld2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - AT_IR_MST_3"]
    #[inline(always)]
    pub fn atirmst3(&self) -> Atirmst3R {
        Atirmst3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - AT_IR_MST_VLD_3"]
    #[inline(always)]
    pub fn atirmstvld3(&self) -> Atirmstvld3R {
        Atirmstvld3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - AT_IR_MST_0"]
    #[inline(always)]
    pub fn atirmst0(&mut self) -> Atirmst0W<At098Spec> {
        Atirmst0W::new(self, 0)
    }
    #[doc = "Bit 7 - AT_IR_MST_VLD_0"]
    #[inline(always)]
    pub fn atirmstvld0(&mut self) -> Atirmstvld0W<At098Spec> {
        Atirmstvld0W::new(self, 7)
    }
    #[doc = "Bits 8:13 - AT_IR_MST_1"]
    #[inline(always)]
    pub fn atirmst1(&mut self) -> Atirmst1W<At098Spec> {
        Atirmst1W::new(self, 8)
    }
    #[doc = "Bit 15 - AT_IR_MST_VLD_1"]
    #[inline(always)]
    pub fn atirmstvld1(&mut self) -> Atirmstvld1W<At098Spec> {
        Atirmstvld1W::new(self, 15)
    }
    #[doc = "Bits 16:21 - AT_IR_MST_2"]
    #[inline(always)]
    pub fn atirmst2(&mut self) -> Atirmst2W<At098Spec> {
        Atirmst2W::new(self, 16)
    }
    #[doc = "Bit 23 - AT_IR_MST_VLD_2"]
    #[inline(always)]
    pub fn atirmstvld2(&mut self) -> Atirmstvld2W<At098Spec> {
        Atirmstvld2W::new(self, 23)
    }
    #[doc = "Bits 24:29 - AT_IR_MST_3"]
    #[inline(always)]
    pub fn atirmst3(&mut self) -> Atirmst3W<At098Spec> {
        Atirmst3W::new(self, 24)
    }
    #[doc = "Bit 31 - AT_IR_MST_VLD_3"]
    #[inline(always)]
    pub fn atirmstvld3(&mut self) -> Atirmstvld3W<At098Spec> {
        Atirmstvld3W::new(self, 31)
    }
}
#[doc = "IR Drop Master\n\nYou can [`read`](crate::Reg::read) this register and get [`at098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At098Spec;
impl crate::RegisterSpec for At098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at098::R`](R) reader structure"]
impl crate::Readable for At098Spec {}
#[doc = "`write(|w| ..)` method takes [`at098::W`](W) writer structure"]
impl crate::Writable for At098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT098 to value 0"]
impl crate::Resettable for At098Spec {}
