#[doc = "Register `AT030` reader"]
pub type R = crate::R<At030Spec>;
#[doc = "Register `AT030` writer"]
pub type W = crate::W<At030Spec>;
#[doc = "Field `ATGDEN` reader - AT_GD_EN"]
pub type AtgdenR = crate::BitReader;
#[doc = "Field `ATGDEN` writer - AT_GD_EN"]
pub type AtgdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDRSTN` reader - AT_GD_RSTN"]
pub type AtgdrstnR = crate::BitReader;
#[doc = "Field `ATGDRSTN` writer - AT_GD_RSTN"]
pub type AtgdrstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDENHYS` reader - AT_GD_EN_HYS"]
pub type AtgdenhysR = crate::BitReader;
#[doc = "Field `ATGDENHYS` writer - AT_GD_EN_HYS"]
pub type AtgdenhysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDTHRESHLO` reader - AT_GD_THRESH_LO"]
pub type AtgdthreshloR = crate::FieldReader;
#[doc = "Field `ATGDTHRESHLO` writer - AT_GD_THRESH_LO"]
pub type AtgdthreshloW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATGDTHRESHHI` reader - AT_GD_THRESH_HI"]
pub type AtgdthreshhiR = crate::FieldReader;
#[doc = "Field `ATGDTHRESHHI` writer - AT_GD_THRESH_HI"]
pub type AtgdthreshhiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATGDTRIMBG` reader - AT_GD_TRIM_BG"]
pub type AtgdtrimbgR = crate::FieldReader;
#[doc = "Field `ATGDTRIMBG` writer - AT_GD_TRIM_BG"]
pub type AtgdtrimbgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATGDTESTMODE` reader - AT_GD_TEST_MODE"]
pub type AtgdtestmodeR = crate::BitReader;
#[doc = "Field `ATGDTESTMODE` writer - AT_GD_TEST_MODE"]
pub type AtgdtestmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATGDMONDIG` reader - AT_GD_MON_DIG"]
pub type AtgdmondigR = crate::FieldReader;
#[doc = "Field `ATGDMONDIG` writer - AT_GD_MON_DIG"]
pub type AtgdmondigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATGDMONANA` reader - AT_GD_MON_ANA"]
pub type AtgdmonanaR = crate::FieldReader;
#[doc = "Field `ATGDMONANA` writer - AT_GD_MON_ANA"]
pub type AtgdmonanaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - AT_GD_EN"]
    #[inline(always)]
    pub fn atgden(&self) -> AtgdenR {
        AtgdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_GD_RSTN"]
    #[inline(always)]
    pub fn atgdrstn(&self) -> AtgdrstnR {
        AtgdrstnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_GD_EN_HYS"]
    #[inline(always)]
    pub fn atgdenhys(&self) -> AtgdenhysR {
        AtgdenhysR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - AT_GD_THRESH_LO"]
    #[inline(always)]
    pub fn atgdthreshlo(&self) -> AtgdthreshloR {
        AtgdthreshloR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - AT_GD_THRESH_HI"]
    #[inline(always)]
    pub fn atgdthreshhi(&self) -> AtgdthreshhiR {
        AtgdthreshhiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:17 - AT_GD_TRIM_BG"]
    #[inline(always)]
    pub fn atgdtrimbg(&self) -> AtgdtrimbgR {
        AtgdtrimbgR::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - AT_GD_TEST_MODE"]
    #[inline(always)]
    pub fn atgdtestmode(&self) -> AtgdtestmodeR {
        AtgdtestmodeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - AT_GD_MON_DIG"]
    #[inline(always)]
    pub fn atgdmondig(&self) -> AtgdmondigR {
        AtgdmondigR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - AT_GD_MON_ANA"]
    #[inline(always)]
    pub fn atgdmonana(&self) -> AtgdmonanaR {
        AtgdmonanaR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AT_GD_EN"]
    #[inline(always)]
    pub fn atgden(&mut self) -> AtgdenW<At030Spec> {
        AtgdenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_GD_RSTN"]
    #[inline(always)]
    pub fn atgdrstn(&mut self) -> AtgdrstnW<At030Spec> {
        AtgdrstnW::new(self, 1)
    }
    #[doc = "Bit 2 - AT_GD_EN_HYS"]
    #[inline(always)]
    pub fn atgdenhys(&mut self) -> AtgdenhysW<At030Spec> {
        AtgdenhysW::new(self, 2)
    }
    #[doc = "Bits 3:7 - AT_GD_THRESH_LO"]
    #[inline(always)]
    pub fn atgdthreshlo(&mut self) -> AtgdthreshloW<At030Spec> {
        AtgdthreshloW::new(self, 3)
    }
    #[doc = "Bits 8:12 - AT_GD_THRESH_HI"]
    #[inline(always)]
    pub fn atgdthreshhi(&mut self) -> AtgdthreshhiW<At030Spec> {
        AtgdthreshhiW::new(self, 8)
    }
    #[doc = "Bits 13:17 - AT_GD_TRIM_BG"]
    #[inline(always)]
    pub fn atgdtrimbg(&mut self) -> AtgdtrimbgW<At030Spec> {
        AtgdtrimbgW::new(self, 13)
    }
    #[doc = "Bit 18 - AT_GD_TEST_MODE"]
    #[inline(always)]
    pub fn atgdtestmode(&mut self) -> AtgdtestmodeW<At030Spec> {
        AtgdtestmodeW::new(self, 18)
    }
    #[doc = "Bits 19:20 - AT_GD_MON_DIG"]
    #[inline(always)]
    pub fn atgdmondig(&mut self) -> AtgdmondigW<At030Spec> {
        AtgdmondigW::new(self, 19)
    }
    #[doc = "Bits 21:23 - AT_GD_MON_ANA"]
    #[inline(always)]
    pub fn atgdmonana(&mut self) -> AtgdmonanaW<At030Spec> {
        AtgdmonanaW::new(self, 21)
    }
}
#[doc = "Glitch Detection Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At030Spec;
impl crate::RegisterSpec for At030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at030::R`](R) reader structure"]
impl crate::Readable for At030Spec {}
#[doc = "`write(|w| ..)` method takes [`at030::W`](W) writer structure"]
impl crate::Writable for At030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT030 to value 0"]
impl crate::Resettable for At030Spec {}
