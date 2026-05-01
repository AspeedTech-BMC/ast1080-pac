#[doc = "Register `HCICAPABILITY02C` reader"]
pub type R = crate::R<Hcicapability02cSpec>;
#[doc = "Register `HCICAPABILITY02C` writer"]
pub type W = crate::W<Hcicapability02cSpec>;
#[doc = "Field `REGHCINTERNALERRFORCE` reader - REG_HC_INTERNAL_ERR_FORCE"]
pub type ReghcinternalerrforceR = crate::BitReader;
#[doc = "Field `REGHCINTERNALERRFORCE` writer - REG_HC_INTERNAL_ERR_FORCE"]
pub type ReghcinternalerrforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCSEQCANCELFORCE` reader - REG_HC_SEQ_CANCEL_FORCE"]
pub type ReghcseqcancelforceR = crate::BitReader;
#[doc = "Field `REGHCSEQCANCELFORCE` writer - REG_HC_SEQ_CANCEL_FORCE"]
pub type ReghcseqcancelforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCWARNCMDSEQSTALLFORCE` reader - REG_HC_WARN_CMD_SEQ_STALL_FORCE"]
pub type ReghcwarncmdseqstallforceR = crate::BitReader;
#[doc = "Field `REGHCWARNCMDSEQSTALLFORCE` writer - REG_HC_WARN_CMD_SEQ_STALL_FORCE"]
pub type ReghcwarncmdseqstallforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTFORCE` reader - REG_HC_ERR_CMD_SEQ_TIMEOUT_FORCE"]
pub type ReghcerrcmdseqtimeoutforceR = crate::BitReader;
#[doc = "Field `REGHCERRCMDSEQTIMEOUTFORCE` writer - REG_HC_ERR_CMD_SEQ_TIMEOUT_FORCE"]
pub type ReghcerrcmdseqtimeoutforceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_FORCE"]
    #[inline(always)]
    pub fn reghcinternalerrforce(&self) -> ReghcinternalerrforceR {
        ReghcinternalerrforceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_FORCE"]
    #[inline(always)]
    pub fn reghcseqcancelforce(&self) -> ReghcseqcancelforceR {
        ReghcseqcancelforceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_FORCE"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallforce(&self) -> ReghcwarncmdseqstallforceR {
        ReghcwarncmdseqstallforceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_FORCE"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutforce(&self) -> ReghcerrcmdseqtimeoutforceR {
        ReghcerrcmdseqtimeoutforceR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - REG_HC_INTERNAL_ERR_FORCE"]
    #[inline(always)]
    pub fn reghcinternalerrforce(&mut self) -> ReghcinternalerrforceW<Hcicapability02cSpec> {
        ReghcinternalerrforceW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_HC_SEQ_CANCEL_FORCE"]
    #[inline(always)]
    pub fn reghcseqcancelforce(&mut self) -> ReghcseqcancelforceW<Hcicapability02cSpec> {
        ReghcseqcancelforceW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_HC_WARN_CMD_SEQ_STALL_FORCE"]
    #[inline(always)]
    pub fn reghcwarncmdseqstallforce(
        &mut self,
    ) -> ReghcwarncmdseqstallforceW<Hcicapability02cSpec> {
        ReghcwarncmdseqstallforceW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_HC_ERR_CMD_SEQ_TIMEOUT_FORCE"]
    #[inline(always)]
    pub fn reghcerrcmdseqtimeoutforce(
        &mut self,
    ) -> ReghcerrcmdseqtimeoutforceW<Hcicapability02cSpec> {
        ReghcerrcmdseqtimeoutforceW::new(self, 13)
    }
}
#[doc = "INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability02cSpec;
impl crate::RegisterSpec for Hcicapability02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability02c::R`](R) reader structure"]
impl crate::Readable for Hcicapability02cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability02c::W`](W) writer structure"]
impl crate::Writable for Hcicapability02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY02C to value 0"]
impl crate::Resettable for Hcicapability02cSpec {}
