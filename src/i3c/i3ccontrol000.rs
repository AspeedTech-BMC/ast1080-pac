#[doc = "Register `I3CCONTROL000` reader"]
pub type R = crate::R<I3ccontrol000Spec>;
#[doc = "Register `I3CCONTROL000` writer"]
pub type W = crate::W<I3ccontrol000Spec>;
#[doc = "Field `REGI3CCTRLINITMODE` reader - REG_I3C_CTRL_INIT_MODE"]
pub type Regi3cctrlinitmodeR = crate::FieldReader;
#[doc = "Field `REGI3CCTRLINITMODE` writer - REG_I3C_CTRL_INIT_MODE"]
pub type Regi3cctrlinitmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGI3CCTRLINITDONE` reader - REG_I3C_CTRL_INIT_DONE"]
pub type Regi3cctrlinitdoneR = crate::BitReader;
#[doc = "Field `REGI3CCTRLINITDONE` writer - REG_I3C_CTRL_INIT_DONE"]
pub type Regi3cctrlinitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGSWRSTWAITCNT` reader - REG_SW_RST_WAIT_CNT"]
pub type RegswrstwaitcntR = crate::FieldReader;
#[doc = "Field `REGSWRSTWAITCNT` writer - REG_SW_RST_WAIT_CNT"]
pub type RegswrstwaitcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGMSTIBIINTOHALT` reader - REG_MST_IBI_INTO_HALT"]
pub type RegmstibiintohaltR = crate::BitReader;
#[doc = "Field `REGMSTIBIINTOHALT` writer - REG_MST_IBI_INTO_HALT"]
pub type RegmstibiintohaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGEARLYSTOPWITHCRC` reader - REG_EARLY_STOP_WITH_CRC"]
pub type RegearlystopwithcrcR = crate::BitReader;
#[doc = "Field `REGEARLYSTOPWITHCRC` writer - REG_EARLY_STOP_WITH_CRC"]
pub type RegearlystopwithcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGENCLOCKSTALL` reader - REG_EN_CLOCK_STALL"]
pub type RegenclockstallR = crate::BitReader;
#[doc = "Field `REGENCLOCKSTALL` writer - REG_EN_CLOCK_STALL"]
pub type RegenclockstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHANDOFFSCLTOGGLE` reader - REG_HANDOFF_SCL_TOGGLE"]
pub type ReghandoffscltoggleR = crate::BitReader;
#[doc = "Field `REGHANDOFFSCLTOGGLE` writer - REG_HANDOFF_SCL_TOGGLE"]
pub type ReghandoffscltoggleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVHANDOFFTOMSTREQ` reader - REG_SLV_HANDOFF_TO_MST_REQ"]
pub type RegslvhandofftomstreqR = crate::BitReader;
#[doc = "Field `REGSLVHANDOFFTOMSTREQ` writer - REG_SLV_HANDOFF_TO_MST_REQ"]
pub type RegslvhandofftomstreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVHANDOFFTOMSTDONE` reader - REG_SLV_HANDOFF_TO_MST_DONE"]
pub type RegslvhandofftomstdoneR = crate::BitReader;
#[doc = "Field `REGSLVFORCEABORT` reader - REG_SLV_FORCE_ABORT"]
pub type RegslvforceabortR = crate::BitReader;
#[doc = "Field `REGSLVFORCEABORT` writer - REG_SLV_FORCE_ABORT"]
pub type RegslvforceabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIAUTOCMDIND` reader - REG_IBI_AUTOCMD_IND"]
pub type RegibiautocmdindR = crate::BitReader;
#[doc = "Field `REGIBIAUTOCMDIND` writer - REG_IBI_AUTOCMD_IND"]
pub type RegibiautocmdindW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIGNORELASTDAA` reader - REG_IGNORE_LAST_DAA"]
pub type RegignorelastdaaR = crate::BitReader;
#[doc = "Field `REGIGNORELASTDAA` writer - REG_IGNORE_LAST_DAA"]
pub type RegignorelastdaaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIRETRYEN` reader - REG_IBI_RETRY_EN"]
pub type RegibiretryenR = crate::BitReader;
#[doc = "Field `REGIBIRETRYEN` writer - REG_IBI_RETRY_EN"]
pub type RegibiretryenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCCCNORESPEN` reader - REG_CCC_NO_RESP_EN"]
pub type RegcccnorespenR = crate::BitReader;
#[doc = "Field `REGCCCNORESPEN` writer - REG_CCC_NO_RESP_EN"]
pub type RegcccnorespenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGCRSTOPFAILNOTINTOHALT` reader - REG_CR_STOP_FAIL_NOT_INTO_HALT"]
pub type RegcrstopfailnotintohaltR = crate::BitReader;
#[doc = "Field `REGCRSTOPFAILNOTINTOHALT` writer - REG_CR_STOP_FAIL_NOT_INTO_HALT"]
pub type RegcrstopfailnotintohaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCONTENTIONNOTINTOHALT` reader - REG_CONTENTION_NOT_INTO_HALT"]
pub type RegcontentionnotintohaltR = crate::BitReader;
#[doc = "Field `REGCONTENTIONNOTINTOHALT` writer - REG_CONTENTION_NOT_INTO_HALT"]
pub type RegcontentionnotintohaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBISRTHENCMD` reader - REG_IBI_SR_THEN_CMD"]
pub type RegibisrthencmdR = crate::BitReader;
#[doc = "Field `REGIBISRTHENCMD` writer - REG_IBI_SR_THEN_CMD"]
pub type RegibisrthencmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTRANSFERABORTHWNOTASSERT` reader - REG_TRANSFER_ABORT_HW_NOT_ASSERT"]
pub type RegtransferaborthwnotassertR = crate::BitReader;
#[doc = "Field `REGTRANSFERABORTHWNOTASSERT` writer - REG_TRANSFER_ABORT_HW_NOT_ASSERT"]
pub type RegtransferaborthwnotassertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGREADTRIGSTOP` reader - REG_READ_TRIG_STOP"]
pub type RegreadtrigstopR = crate::BitReader;
#[doc = "Field `REGREADTRIGSTOP` writer - REG_READ_TRIG_STOP"]
pub type RegreadtrigstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - REG_I3C_CTRL_INIT_MODE"]
    #[inline(always)]
    pub fn regi3cctrlinitmode(&self) -> Regi3cctrlinitmodeR {
        Regi3cctrlinitmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_I3C_CTRL_INIT_DONE"]
    #[inline(always)]
    pub fn regi3cctrlinitdone(&self) -> Regi3cctrlinitdoneR {
        Regi3cctrlinitdoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - REG_SW_RST_WAIT_CNT"]
    #[inline(always)]
    pub fn regswrstwaitcnt(&self) -> RegswrstwaitcntR {
        RegswrstwaitcntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - REG_MST_IBI_INTO_HALT"]
    #[inline(always)]
    pub fn regmstibiintohalt(&self) -> RegmstibiintohaltR {
        RegmstibiintohaltR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_EARLY_STOP_WITH_CRC"]
    #[inline(always)]
    pub fn regearlystopwithcrc(&self) -> RegearlystopwithcrcR {
        RegearlystopwithcrcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - REG_EN_CLOCK_STALL"]
    #[inline(always)]
    pub fn regenclockstall(&self) -> RegenclockstallR {
        RegenclockstallR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REG_HANDOFF_SCL_TOGGLE"]
    #[inline(always)]
    pub fn reghandoffscltoggle(&self) -> ReghandoffscltoggleR {
        ReghandoffscltoggleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - REG_SLV_HANDOFF_TO_MST_REQ"]
    #[inline(always)]
    pub fn regslvhandofftomstreq(&self) -> RegslvhandofftomstreqR {
        RegslvhandofftomstreqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - REG_SLV_HANDOFF_TO_MST_DONE"]
    #[inline(always)]
    pub fn regslvhandofftomstdone(&self) -> RegslvhandofftomstdoneR {
        RegslvhandofftomstdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - REG_SLV_FORCE_ABORT"]
    #[inline(always)]
    pub fn regslvforceabort(&self) -> RegslvforceabortR {
        RegslvforceabortR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - REG_IBI_AUTOCMD_IND"]
    #[inline(always)]
    pub fn regibiautocmdind(&self) -> RegibiautocmdindR {
        RegibiautocmdindR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REG_IGNORE_LAST_DAA"]
    #[inline(always)]
    pub fn regignorelastdaa(&self) -> RegignorelastdaaR {
        RegignorelastdaaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - REG_IBI_RETRY_EN"]
    #[inline(always)]
    pub fn regibiretryen(&self) -> RegibiretryenR {
        RegibiretryenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - REG_CCC_NO_RESP_EN"]
    #[inline(always)]
    pub fn regcccnorespen(&self) -> RegcccnorespenR {
        RegcccnorespenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - REG_CR_STOP_FAIL_NOT_INTO_HALT"]
    #[inline(always)]
    pub fn regcrstopfailnotintohalt(&self) -> RegcrstopfailnotintohaltR {
        RegcrstopfailnotintohaltR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - REG_CONTENTION_NOT_INTO_HALT"]
    #[inline(always)]
    pub fn regcontentionnotintohalt(&self) -> RegcontentionnotintohaltR {
        RegcontentionnotintohaltR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - REG_IBI_SR_THEN_CMD"]
    #[inline(always)]
    pub fn regibisrthencmd(&self) -> RegibisrthencmdR {
        RegibisrthencmdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - REG_TRANSFER_ABORT_HW_NOT_ASSERT"]
    #[inline(always)]
    pub fn regtransferaborthwnotassert(&self) -> RegtransferaborthwnotassertR {
        RegtransferaborthwnotassertR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - REG_READ_TRIG_STOP"]
    #[inline(always)]
    pub fn regreadtrigstop(&self) -> RegreadtrigstopR {
        RegreadtrigstopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - REG_I3C_CTRL_INIT_MODE"]
    #[inline(always)]
    pub fn regi3cctrlinitmode(&mut self) -> Regi3cctrlinitmodeW<I3ccontrol000Spec> {
        Regi3cctrlinitmodeW::new(self, 0)
    }
    #[doc = "Bit 4 - REG_I3C_CTRL_INIT_DONE"]
    #[inline(always)]
    pub fn regi3cctrlinitdone(&mut self) -> Regi3cctrlinitdoneW<I3ccontrol000Spec> {
        Regi3cctrlinitdoneW::new(self, 4)
    }
    #[doc = "Bits 8:11 - REG_SW_RST_WAIT_CNT"]
    #[inline(always)]
    pub fn regswrstwaitcnt(&mut self) -> RegswrstwaitcntW<I3ccontrol000Spec> {
        RegswrstwaitcntW::new(self, 8)
    }
    #[doc = "Bit 12 - REG_MST_IBI_INTO_HALT"]
    #[inline(always)]
    pub fn regmstibiintohalt(&mut self) -> RegmstibiintohaltW<I3ccontrol000Spec> {
        RegmstibiintohaltW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_EARLY_STOP_WITH_CRC"]
    #[inline(always)]
    pub fn regearlystopwithcrc(&mut self) -> RegearlystopwithcrcW<I3ccontrol000Spec> {
        RegearlystopwithcrcW::new(self, 13)
    }
    #[doc = "Bit 14 - REG_EN_CLOCK_STALL"]
    #[inline(always)]
    pub fn regenclockstall(&mut self) -> RegenclockstallW<I3ccontrol000Spec> {
        RegenclockstallW::new(self, 14)
    }
    #[doc = "Bit 15 - REG_HANDOFF_SCL_TOGGLE"]
    #[inline(always)]
    pub fn reghandoffscltoggle(&mut self) -> ReghandoffscltoggleW<I3ccontrol000Spec> {
        ReghandoffscltoggleW::new(self, 15)
    }
    #[doc = "Bit 16 - REG_SLV_HANDOFF_TO_MST_REQ"]
    #[inline(always)]
    pub fn regslvhandofftomstreq(&mut self) -> RegslvhandofftomstreqW<I3ccontrol000Spec> {
        RegslvhandofftomstreqW::new(self, 16)
    }
    #[doc = "Bit 18 - REG_SLV_FORCE_ABORT"]
    #[inline(always)]
    pub fn regslvforceabort(&mut self) -> RegslvforceabortW<I3ccontrol000Spec> {
        RegslvforceabortW::new(self, 18)
    }
    #[doc = "Bit 19 - REG_IBI_AUTOCMD_IND"]
    #[inline(always)]
    pub fn regibiautocmdind(&mut self) -> RegibiautocmdindW<I3ccontrol000Spec> {
        RegibiautocmdindW::new(self, 19)
    }
    #[doc = "Bit 20 - REG_IGNORE_LAST_DAA"]
    #[inline(always)]
    pub fn regignorelastdaa(&mut self) -> RegignorelastdaaW<I3ccontrol000Spec> {
        RegignorelastdaaW::new(self, 20)
    }
    #[doc = "Bit 21 - REG_IBI_RETRY_EN"]
    #[inline(always)]
    pub fn regibiretryen(&mut self) -> RegibiretryenW<I3ccontrol000Spec> {
        RegibiretryenW::new(self, 21)
    }
    #[doc = "Bit 22 - REG_CCC_NO_RESP_EN"]
    #[inline(always)]
    pub fn regcccnorespen(&mut self) -> RegcccnorespenW<I3ccontrol000Spec> {
        RegcccnorespenW::new(self, 22)
    }
    #[doc = "Bit 24 - REG_CR_STOP_FAIL_NOT_INTO_HALT"]
    #[inline(always)]
    pub fn regcrstopfailnotintohalt(&mut self) -> RegcrstopfailnotintohaltW<I3ccontrol000Spec> {
        RegcrstopfailnotintohaltW::new(self, 24)
    }
    #[doc = "Bit 25 - REG_CONTENTION_NOT_INTO_HALT"]
    #[inline(always)]
    pub fn regcontentionnotintohalt(&mut self) -> RegcontentionnotintohaltW<I3ccontrol000Spec> {
        RegcontentionnotintohaltW::new(self, 25)
    }
    #[doc = "Bit 26 - REG_IBI_SR_THEN_CMD"]
    #[inline(always)]
    pub fn regibisrthencmd(&mut self) -> RegibisrthencmdW<I3ccontrol000Spec> {
        RegibisrthencmdW::new(self, 26)
    }
    #[doc = "Bit 27 - REG_TRANSFER_ABORT_HW_NOT_ASSERT"]
    #[inline(always)]
    pub fn regtransferaborthwnotassert(
        &mut self,
    ) -> RegtransferaborthwnotassertW<I3ccontrol000Spec> {
        RegtransferaborthwnotassertW::new(self, 27)
    }
    #[doc = "Bit 31 - REG_READ_TRIG_STOP"]
    #[inline(always)]
    pub fn regreadtrigstop(&mut self) -> RegreadtrigstopW<I3ccontrol000Spec> {
        RegreadtrigstopW::new(self, 31)
    }
}
#[doc = "I3C\\_CONTROL\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol000Spec;
impl crate::RegisterSpec for I3ccontrol000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol000::R`](R) reader structure"]
impl crate::Readable for I3ccontrol000Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol000::W`](W) writer structure"]
impl crate::Writable for I3ccontrol000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL000 to value 0x2400"]
impl crate::Resettable for I3ccontrol000Spec {
    const RESET_VALUE: u32 = 0x2400;
}
