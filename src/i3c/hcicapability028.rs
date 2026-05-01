#[doc = "Register `HCICAPABILITY028` reader"]
pub type R = crate::R<Hcicapability028Spec>;
#[doc = "Register `HCICAPABILITY028` writer"]
pub type W = crate::W<Hcicapability028Spec>;
#[doc = "Field `REGHCINTERNALERRSIGNALEN` reader - REG_HC_INTERNAL_ERR_SIGNAL_EN"]
pub type ReghcinternalerrsignalenR = crate::BitReader;
#[doc = "Field `REGHCINTERNALERRSIGNALEN` writer - REG_HC_INTERNAL_ERR_SIGNAL_EN"]
pub type ReghcinternalerrsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCSEQCANCELSIGNALEN` reader - REG_HC_SEQ_CANCEL_SIGNAL_EN"]
pub type ReghcseqcancelsignalenR = crate::BitReader;
#[doc = "Field `REGHCSEQCANCELSIGNALEN` writer - REG_HC_SEQ_CANCEL_SIGNAL_EN"]
pub type ReghcseqcancelsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCWARNCMDSEQSTALLSIGNALEN` reader - REG_HC_WARN_CMD_SEQ_STALL_SIGNAL_EN"]
pub type ReghcwarncmdseqstallsignalenR = crate::BitReader;
#[doc = "Field `REGHCWARNCMDSEQSTALLSIGNALEN` writer - REG_HC_WARN_CMD_SEQ_STALL_SIGNAL_EN"]
pub type ReghcwarncmdseqstallsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTSIGNALEN` reader - REG_HC_ERR_CMD_SEQ_TIMEOUT_SIGNAL_EN"]
pub type ReghcerrcmdseqtimeoutsignalenR = crate::BitReader;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTSIGNALEN` writer - REG_HC_ERR_CMD_SEQ_TIMEOUT_SIGNAL_EN"]
pub type ReghcerrcmdseqtimeoutsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcinternalerrsignalen(&self) -> ReghcinternalerrsignalenR {
        ReghcinternalerrsignalenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcseqcancelsignalen(&self) -> ReghcseqcancelsignalenR {
        ReghcseqcancelsignalenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallsignalen(&self) -> ReghcwarncmdseqstallsignalenR {
        ReghcwarncmdseqstallsignalenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutsignalen(&self) -> ReghcerrcmdseqtimeoutsignalenR {
        ReghcerrcmdseqtimeoutsignalenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcinternalerrsignalen(&mut self) -> ReghcinternalerrsignalenW<Hcicapability028Spec> {
        ReghcinternalerrsignalenW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcseqcancelsignalen(&mut self) -> ReghcseqcancelsignalenW<Hcicapability028Spec> {
        ReghcseqcancelsignalenW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallsignalen(
        &mut self,
    ) -> ReghcwarncmdseqstallsignalenW<Hcicapability028Spec> {
        ReghcwarncmdseqstallsignalenW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_SIGNAL_EN"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutsignalen(
        &mut self,
    ) -> ReghcerrcmdseqtimeoutsignalenW<Hcicapability028Spec> {
        ReghcerrcmdseqtimeoutsignalenW::new(self, 13)
    }
}
#[doc = "INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability028Spec;
impl crate::RegisterSpec for Hcicapability028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability028::R`](R) reader structure"]
impl crate::Readable for Hcicapability028Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability028::W`](W) writer structure"]
impl crate::Writable for Hcicapability028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY028 to value 0"]
impl crate::Resettable for Hcicapability028Spec {}
