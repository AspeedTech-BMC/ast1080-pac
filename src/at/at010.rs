#[doc = "Register `AT010` reader"]
pub type R = crate::R<At010Spec>;
#[doc = "Register `AT010` writer"]
pub type W = crate::W<At010Spec>;
#[doc = "Field `ATCAMEN` reader - AT_CAM_EN"]
pub type AtcamenR = crate::BitReader;
#[doc = "Field `ATCAMEN` writer - AT_CAM_EN"]
pub type AtcamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCAMRSTN` reader - AT_CAM_RSTN"]
pub type AtcamrstnR = crate::BitReader;
#[doc = "Field `ATCAMRSTN` writer - AT_CAM_RSTN"]
pub type AtcamrstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `ATCAMTRIMBG` reader - AT_CAM_TRIM_BG"]
pub type AtcamtrimbgR = crate::FieldReader;
#[doc = "Field `ATCAMTRIMBG` writer - AT_CAM_TRIM_BG"]
pub type AtcamtrimbgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATCAMCALVAL` reader - AT_CAM_CALVAL"]
pub type AtcamcalvalR = crate::FieldReader;
#[doc = "Field `ATCAMCALVAL` writer - AT_CAM_CALVAL"]
pub type AtcamcalvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATCAMEARLYOFFSET` reader - AT_CAM_EARLY_OFFSET"]
pub type AtcamearlyoffsetR = crate::FieldReader;
#[doc = "Field `ATCAMEARLYOFFSET` writer - AT_CAM_EARLY_OFFSET"]
pub type AtcamearlyoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATCAMLATEOFFSET` reader - AT_CAM_LATE_OFFSET"]
pub type AtcamlateoffsetR = crate::FieldReader;
#[doc = "Field `ATCAMLATEOFFSET` writer - AT_CAM_LATE_OFFSET"]
pub type AtcamlateoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATCAMSELMON` reader - AT_CAM_SEL_MON"]
pub type AtcamselmonR = crate::FieldReader;
#[doc = "Field `ATCAMSELMON` writer - AT_CAM_SEL_MON"]
pub type AtcamselmonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - AT_CAM_EN"]
    #[inline(always)]
    pub fn atcamen(&self) -> AtcamenR {
        AtcamenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_CAM_RSTN"]
    #[inline(always)]
    pub fn atcamrstn(&self) -> AtcamrstnR {
        AtcamrstnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:8 - AT_CAM_TRIM_BG"]
    #[inline(always)]
    pub fn atcamtrimbg(&self) -> AtcamtrimbgR {
        AtcamtrimbgR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:19 - AT_CAM_CALVAL"]
    #[inline(always)]
    pub fn atcamcalval(&self) -> AtcamcalvalR {
        AtcamcalvalR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:24 - AT_CAM_EARLY_OFFSET"]
    #[inline(always)]
    pub fn atcamearlyoffset(&self) -> AtcamearlyoffsetR {
        AtcamearlyoffsetR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - AT_CAM_LATE_OFFSET"]
    #[inline(always)]
    pub fn atcamlateoffset(&self) -> AtcamlateoffsetR {
        AtcamlateoffsetR::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - AT_CAM_SEL_MON"]
    #[inline(always)]
    pub fn atcamselmon(&self) -> AtcamselmonR {
        AtcamselmonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AT_CAM_EN"]
    #[inline(always)]
    pub fn atcamen(&mut self) -> AtcamenW<At010Spec> {
        AtcamenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_CAM_RSTN"]
    #[inline(always)]
    pub fn atcamrstn(&mut self) -> AtcamrstnW<At010Spec> {
        AtcamrstnW::new(self, 1)
    }
    #[doc = "Bits 4:8 - AT_CAM_TRIM_BG"]
    #[inline(always)]
    pub fn atcamtrimbg(&mut self) -> AtcamtrimbgW<At010Spec> {
        AtcamtrimbgW::new(self, 4)
    }
    #[doc = "Bits 12:19 - AT_CAM_CALVAL"]
    #[inline(always)]
    pub fn atcamcalval(&mut self) -> AtcamcalvalW<At010Spec> {
        AtcamcalvalW::new(self, 12)
    }
    #[doc = "Bits 20:24 - AT_CAM_EARLY_OFFSET"]
    #[inline(always)]
    pub fn atcamearlyoffset(&mut self) -> AtcamearlyoffsetW<At010Spec> {
        AtcamearlyoffsetW::new(self, 20)
    }
    #[doc = "Bits 25:29 - AT_CAM_LATE_OFFSET"]
    #[inline(always)]
    pub fn atcamlateoffset(&mut self) -> AtcamlateoffsetW<At010Spec> {
        AtcamlateoffsetW::new(self, 25)
    }
    #[doc = "Bits 30:31 - AT_CAM_SEL_MON"]
    #[inline(always)]
    pub fn atcamselmon(&mut self) -> AtcamselmonW<At010Spec> {
        AtcamselmonW::new(self, 30)
    }
}
#[doc = "Clock Attack Monitor Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At010Spec;
impl crate::RegisterSpec for At010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at010::R`](R) reader structure"]
impl crate::Readable for At010Spec {}
#[doc = "`write(|w| ..)` method takes [`at010::W`](W) writer structure"]
impl crate::Writable for At010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT010 to value 0"]
impl crate::Resettable for At010Spec {}
