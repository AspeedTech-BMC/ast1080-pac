#[doc = "Register `HCICAPABILITY024` reader"]
pub type R = crate::R<Hcicapability024Spec>;
#[doc = "Register `HCICAPABILITY024` writer"]
pub type W = crate::W<Hcicapability024Spec>;
#[doc = "Field `REGHCINTERNALERRSTATEN` reader - REG_HC_INTERNAL_ERR_STAT_EN"]
pub type ReghcinternalerrstatenR = crate::BitReader;
#[doc = "Field `REGHCINTERNALERRSTATEN` writer - REG_HC_INTERNAL_ERR_STAT_EN"]
pub type ReghcinternalerrstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCSEQCANCELSTATEN` reader - REG_HC_SEQ_CANCEL_STAT_EN"]
pub type ReghcseqcancelstatenR = crate::BitReader;
#[doc = "Field `REGHCSEQCANCELSTATEN` writer - REG_HC_SEQ_CANCEL_STAT_EN"]
pub type ReghcseqcancelstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCWARNCMDSEQSTALLSTATEN` reader - REG_HC_WARN_CMD_SEQ_STALL_STAT_EN"]
pub type ReghcwarncmdseqstallstatenR = crate::BitReader;
#[doc = "Field `REGHCWARNCMDSEQSTALLSTATEN` writer - REG_HC_WARN_CMD_SEQ_STALL_STAT_EN"]
pub type ReghcwarncmdseqstallstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTSTATEN` reader - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT_EN"]
pub type ReghcerrcmdseqtimeoutstatenR = crate::BitReader;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTSTATEN` writer - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT_EN"]
pub type ReghcerrcmdseqtimeoutstatenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_STAT_EN"]
    #[inline(always)]
    pub fn reghcinternalerrstaten(&self) -> ReghcinternalerrstatenR {
        ReghcinternalerrstatenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_STAT_EN"]
    #[inline(always)]
    pub fn reghcseqcancelstaten(&self) -> ReghcseqcancelstatenR {
        ReghcseqcancelstatenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_STAT_EN"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallstaten(&self) -> ReghcwarncmdseqstallstatenR {
        ReghcwarncmdseqstallstatenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT_EN"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutstaten(&self) -> ReghcerrcmdseqtimeoutstatenR {
        ReghcerrcmdseqtimeoutstatenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_STAT_EN"]
    #[inline(always)]
    pub fn reghcinternalerrstaten(&mut self) -> ReghcinternalerrstatenW<Hcicapability024Spec> {
        ReghcinternalerrstatenW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_STAT_EN"]
    #[inline(always)]
    pub fn reghcseqcancelstaten(&mut self) -> ReghcseqcancelstatenW<Hcicapability024Spec> {
        ReghcseqcancelstatenW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_STAT_EN"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallstaten(
        &mut self,
    ) -> ReghcwarncmdseqstallstatenW<Hcicapability024Spec> {
        ReghcwarncmdseqstallstatenW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_STAT_EN"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutstaten(
        &mut self,
    ) -> ReghcerrcmdseqtimeoutstatenW<Hcicapability024Spec> {
        ReghcerrcmdseqtimeoutstatenW::new(self, 13)
    }
}
#[doc = "INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability024Spec;
impl crate::RegisterSpec for Hcicapability024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability024::R`](R) reader structure"]
impl crate::Readable for Hcicapability024Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability024::W`](W) writer structure"]
impl crate::Writable for Hcicapability024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY024 to value 0"]
impl crate::Resettable for Hcicapability024Spec {}
