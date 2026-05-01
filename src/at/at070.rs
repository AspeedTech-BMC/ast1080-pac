#[doc = "Register `AT070` reader"]
pub type R = crate::R<At070Spec>;
#[doc = "Register `AT070` writer"]
pub type W = crate::W<At070Spec>;
#[doc = "Field `ATTDEN` reader - AT_TD_EN"]
pub type AttdenR = crate::BitReader;
#[doc = "Field `ATTDEN` writer - AT_TD_EN"]
pub type AttdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDRSTN` reader - AT_TD_RSTN"]
pub type AttdrstnR = crate::BitReader;
#[doc = "Field `ATTDRSTN` writer - AT_TD_RSTN"]
pub type AttdrstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDCORR` reader - AT_TD_CORR"]
pub type AttdcorrR = crate::BitReader;
#[doc = "Field `ATTDCORR` writer - AT_TD_CORR"]
pub type AttdcorrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDSTART` reader - AT_TD_START"]
pub type AttdstartR = crate::BitReader;
#[doc = "Field `ATTDSTART` writer - AT_TD_START"]
pub type AttdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDMODE` reader - AT_TD_MODE"]
pub type AttdmodeR = crate::BitReader;
#[doc = "Field `ATTDMODE` writer - AT_TD_MODE"]
pub type AttdmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATTDOFFSET` reader - AT_TD_OFFSET"]
pub type AttdoffsetR = crate::FieldReader;
#[doc = "Field `ATTDOFFSET` writer - AT_TD_OFFSET"]
pub type AttdoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTDSELMON` reader - AT_TD_SEL_MON"]
pub type AttdselmonR = crate::FieldReader;
#[doc = "Field `ATTDSELMON` writer - AT_TD_SEL_MON"]
pub type AttdselmonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - AT_TD_EN"]
    #[inline(always)]
    pub fn attden(&self) -> AttdenR {
        AttdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AT_TD_RSTN"]
    #[inline(always)]
    pub fn attdrstn(&self) -> AttdrstnR {
        AttdrstnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AT_TD_CORR"]
    #[inline(always)]
    pub fn attdcorr(&self) -> AttdcorrR {
        AttdcorrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AT_TD_START"]
    #[inline(always)]
    pub fn attdstart(&self) -> AttdstartR {
        AttdstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AT_TD_MODE"]
    #[inline(always)]
    pub fn attdmode(&self) -> AttdmodeR {
        AttdmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 22:29 - AT_TD_OFFSET"]
    #[inline(always)]
    pub fn attdoffset(&self) -> AttdoffsetR {
        AttdoffsetR::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - AT_TD_SEL_MON"]
    #[inline(always)]
    pub fn attdselmon(&self) -> AttdselmonR {
        AttdselmonR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AT_TD_EN"]
    #[inline(always)]
    pub fn attden(&mut self) -> AttdenW<At070Spec> {
        AttdenW::new(self, 0)
    }
    #[doc = "Bit 1 - AT_TD_RSTN"]
    #[inline(always)]
    pub fn attdrstn(&mut self) -> AttdrstnW<At070Spec> {
        AttdrstnW::new(self, 1)
    }
    #[doc = "Bit 2 - AT_TD_CORR"]
    #[inline(always)]
    pub fn attdcorr(&mut self) -> AttdcorrW<At070Spec> {
        AttdcorrW::new(self, 2)
    }
    #[doc = "Bit 3 - AT_TD_START"]
    #[inline(always)]
    pub fn attdstart(&mut self) -> AttdstartW<At070Spec> {
        AttdstartW::new(self, 3)
    }
    #[doc = "Bit 4 - AT_TD_MODE"]
    #[inline(always)]
    pub fn attdmode(&mut self) -> AttdmodeW<At070Spec> {
        AttdmodeW::new(self, 4)
    }
    #[doc = "Bits 22:29 - AT_TD_OFFSET"]
    #[inline(always)]
    pub fn attdoffset(&mut self) -> AttdoffsetW<At070Spec> {
        AttdoffsetW::new(self, 22)
    }
    #[doc = "Bits 30:31 - AT_TD_SEL_MON"]
    #[inline(always)]
    pub fn attdselmon(&mut self) -> AttdselmonW<At070Spec> {
        AttdselmonW::new(self, 30)
    }
}
#[doc = "TSENSE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`at070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct At070Spec;
impl crate::RegisterSpec for At070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at070::R`](R) reader structure"]
impl crate::Readable for At070Spec {}
#[doc = "`write(|w| ..)` method takes [`at070::W`](W) writer structure"]
impl crate::Writable for At070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT070 to value 0"]
impl crate::Resettable for At070Spec {}
