#[doc = "Register `HCICAPABILITY020` reader"]
pub type R = crate::R<Hcicapability020Spec>;
#[doc = "Register `HCICAPABILITY020` writer"]
pub type W = crate::W<Hcicapability020Spec>;
#[doc = "Field `REGHCINTERNALERRSTAT` reader - REG_HC_INTERNAL_ERR_STAT"]
pub type ReghcinternalerrstatR = crate::BitReader;
#[doc = "Field `REGHCINTERNALERRSTAT` writer - REG_HC_INTERNAL_ERR_STAT"]
pub type ReghcinternalerrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCSEQCANCELSTAT` reader - REG_HC_SEQ_CANCEL_STAT"]
pub type ReghcseqcancelstatR = crate::BitReader;
#[doc = "Field `REGHCSEQCANCELSTAT` writer - REG_HC_SEQ_CANCEL_STAT"]
pub type ReghcseqcancelstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCWARNCMDSEQSTALLSTAT` reader - REG_HC_WARN_CMD_SEQ_STALL_STAT"]
pub type ReghcwarncmdseqstallstatR = crate::BitReader;
#[doc = "Field `REGHCWARNCMDSEQSTALLSTAT` writer - REG_HC_WARN_CMD_SEQ_STALL_STAT"]
pub type ReghcwarncmdseqstallstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTSTAT` reader - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT"]
pub type ReghcerrcmdseqtimeoutstatR = crate::BitReader;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTSTAT` writer - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT"]
pub type ReghcerrcmdseqtimeoutstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_STAT"]
    #[inline(always)]
    pub fn reghcinternalerrstat(&self) -> ReghcinternalerrstatR {
        ReghcinternalerrstatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_STAT"]
    #[inline(always)]
    pub fn reghcseqcancelstat(&self) -> ReghcseqcancelstatR {
        ReghcseqcancelstatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_STAT"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallstat(&self) -> ReghcwarncmdseqstallstatR {
        ReghcwarncmdseqstallstatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutstat(&self) -> ReghcerrcmdseqtimeoutstatR {
        ReghcerrcmdseqtimeoutstatR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_STAT"]
    #[inline(always)]
    pub fn reghcinternalerrstat(&mut self) -> ReghcinternalerrstatW<Hcicapability020Spec> {
        ReghcinternalerrstatW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_STAT"]
    #[inline(always)]
    pub fn reghcseqcancelstat(&mut self) -> ReghcseqcancelstatW<Hcicapability020Spec> {
        ReghcseqcancelstatW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_STAT"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallstat(&mut self) -> ReghcwarncmdseqstallstatW<Hcicapability020Spec> {
        ReghcwarncmdseqstallstatW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutstat(
        &mut self,
    ) -> ReghcerrcmdseqtimeoutstatW<Hcicapability020Spec> {
        ReghcerrcmdseqtimeoutstatW::new(self, 13)
    }
}
#[doc = "INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability020Spec;
impl crate::RegisterSpec for Hcicapability020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability020::R`](R) reader structure"]
impl crate::Readable for Hcicapability020Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability020::W`](W) writer structure"]
impl crate::Writable for Hcicapability020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY020 to value 0"]
impl crate::Resettable for Hcicapability020Spec {}
