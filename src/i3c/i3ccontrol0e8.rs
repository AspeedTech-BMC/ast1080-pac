#[doc = "Register `I3CCONTROL0E8` reader"]
pub type R = crate::R<I3ccontrol0e8Spec>;
#[doc = "Register `I3CCONTROL0E8` writer"]
pub type W = crate::W<I3ccontrol0e8Spec>;
#[doc = "Field `REGTGRSTSIGNALEN` reader - REG_TGRST_SIGNAL_EN"]
pub type RegtgrstsignalenR = crate::BitReader;
#[doc = "Field `REGTGRSTSIGNALEN` writer - REG_TGRST_SIGNAL_EN"]
pub type RegtgrstsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVSCLSTUCKSIGNALEN` reader - REG_SLV_SCL_STUCK_SIGNAL_EN"]
pub type RegslvsclstucksignalenR = crate::BitReader;
#[doc = "Field `REGSLVSCLSTUCKSIGNALEN` writer - REG_SLV_SCL_STUCK_SIGNAL_EN"]
pub type RegslvsclstucksignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGMSTDAADONESIGNALEN` reader - REG_MST_DAA_DONE_SIGNAL_EN"]
pub type RegmstdaadonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTDAADONESIGNALEN` writer - REG_MST_DAA_DONE_SIGNAL_EN"]
pub type RegmstdaadonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTWRITEDONESIGNALEN` reader - REG_MST_WRITE_DONE_SIGNAL_EN"]
pub type RegmstwritedonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTWRITEDONESIGNALEN` writer - REG_MST_WRITE_DONE_SIGNAL_EN"]
pub type RegmstwritedonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTREADDONESIGNALEN` reader - REG_MST_READ_DONE_SIGNAL_EN"]
pub type RegmstreaddonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTREADDONESIGNALEN` writer - REG_MST_READ_DONE_SIGNAL_EN"]
pub type RegmstreaddonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTIBIDONESIGNALEN` reader - REG_MST_IBI_DONE_SIGNAL_EN"]
pub type RegmstibidonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTIBIDONESIGNALEN` writer - REG_MST_IBI_DONE_SIGNAL_EN"]
pub type RegmstibidonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRWRITEDONESIGNALEN` reader - REG_MST_DDR_WRITE_DONE_SIGNAL_EN"]
pub type RegmstddrwritedonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTDDRWRITEDONESIGNALEN` writer - REG_MST_DDR_WRITE_DONE_SIGNAL_EN"]
pub type RegmstddrwritedonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRREADDONESIGNALEN` reader - REG_MST_DDR_READ_DONE_SIGNAL_EN"]
pub type RegmstddrreaddonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTDDRREADDONESIGNALEN` writer - REG_MST_DDR_READ_DONE_SIGNAL_EN"]
pub type RegmstddrreaddonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTINTERNALDONESIGNALEN` reader - REG_MST_INTERNAL_DONE_SIGNAL_EN"]
pub type RegmstinternaldonesignalenR = crate::BitReader;
#[doc = "Field `REGMSTINTERNALDONESIGNALEN` writer - REG_MST_INTERNAL_DONE_SIGNAL_EN"]
pub type RegmstinternaldonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGDETCRSTOPFAILSIGNALEN` reader - REG_DET_CR_STOP_FAIL_SIGNAL_EN"]
pub type RegdetcrstopfailsignalenR = crate::BitReader;
#[doc = "Field `REGDETCRSTOPFAILSIGNALEN` writer - REG_DET_CR_STOP_FAIL_SIGNAL_EN"]
pub type RegdetcrstopfailsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGROLECHANGESIGNALEN` reader - REG_ROLE_CHANGE_SIGNAL_EN"]
pub type RegrolechangesignalenR = crate::BitReader;
#[doc = "Field `REGROLECHANGESIGNALEN` writer - REG_ROLE_CHANGE_SIGNAL_EN"]
pub type RegrolechangesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSDRBUSCDSIGNALEN` reader - REG_SDR_BUS_CD_SIGNAL_EN"]
pub type RegsdrbuscdsignalenR = crate::BitReader;
#[doc = "Field `REGSDRBUSCDSIGNALEN` writer - REG_SDR_BUS_CD_SIGNAL_EN"]
pub type RegsdrbuscdsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDDRBUSCDSIGNALEN` reader - REG_DDR_BUS_CD_SIGNAL_EN"]
pub type RegddrbuscdsignalenR = crate::BitReader;
#[doc = "Field `REGDDRBUSCDSIGNALEN` writer - REG_DDR_BUS_CD_SIGNAL_EN"]
pub type RegddrbuscdsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFSIGNALEN` reader - REG_CR_DEQ_PTR_DIFF_SIGNAL_EN"]
pub type RegcrdeqptrdiffsignalenR = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFSIGNALEN` writer - REG_CR_DEQ_PTR_DIFF_SIGNAL_EN"]
pub type RegcrdeqptrdiffsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIENQPTRDIFFSIGNALEN` reader - REG_IBI_ENQ_PTR_DIFF_SIGNAL_EN"]
pub type RegibienqptrdiffsignalenR = crate::BitReader;
#[doc = "Field `REGIBIENQPTRDIFFSIGNALEN` writer - REG_IBI_ENQ_PTR_DIFF_SIGNAL_EN"]
pub type RegibienqptrdiffsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_TGRST_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtgrstsignalen(&self) -> RegtgrstsignalenR {
        RegtgrstsignalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_SIGNAL_EN"]
    #[inline(always)]
    pub fn regslvsclstucksignalen(&self) -> RegslvsclstucksignalenR {
        RegslvsclstucksignalenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstdaadonesignalen(&self) -> RegmstdaadonesignalenR {
        RegmstdaadonesignalenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstwritedonesignalen(&self) -> RegmstwritedonesignalenR {
        RegmstwritedonesignalenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstreaddonesignalen(&self) -> RegmstreaddonesignalenR {
        RegmstreaddonesignalenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstibidonesignalen(&self) -> RegmstibidonesignalenR {
        RegmstibidonesignalenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstddrwritedonesignalen(&self) -> RegmstddrwritedonesignalenR {
        RegmstddrwritedonesignalenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstddrreaddonesignalen(&self) -> RegmstddrreaddonesignalenR {
        RegmstddrreaddonesignalenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstinternaldonesignalen(&self) -> RegmstinternaldonesignalenR {
        RegmstinternaldonesignalenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_SIGNAL_EN"]
    #[inline(always)]
    pub fn regdetcrstopfailsignalen(&self) -> RegdetcrstopfailsignalenR {
        RegdetcrstopfailsignalenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regrolechangesignalen(&self) -> RegrolechangesignalenR {
        RegrolechangesignalenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regsdrbuscdsignalen(&self) -> RegsdrbuscdsignalenR {
        RegsdrbuscdsignalenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regddrbuscdsignalen(&self) -> RegddrbuscdsignalenR {
        RegddrbuscdsignalenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_SIGNAL_EN"]
    #[inline(always)]
    pub fn regcrdeqptrdiffsignalen(&self) -> RegcrdeqptrdiffsignalenR {
        RegcrdeqptrdiffsignalenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_SIGNAL_EN"]
    #[inline(always)]
    pub fn regibienqptrdiffsignalen(&self) -> RegibienqptrdiffsignalenR {
        RegibienqptrdiffsignalenR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_TGRST_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtgrstsignalen(&mut self) -> RegtgrstsignalenW<I3ccontrol0e8Spec> {
        RegtgrstsignalenW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_SIGNAL_EN"]
    #[inline(always)]
    pub fn regslvsclstucksignalen(&mut self) -> RegslvsclstucksignalenW<I3ccontrol0e8Spec> {
        RegslvsclstucksignalenW::new(self, 1)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstdaadonesignalen(&mut self) -> RegmstdaadonesignalenW<I3ccontrol0e8Spec> {
        RegmstdaadonesignalenW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstwritedonesignalen(&mut self) -> RegmstwritedonesignalenW<I3ccontrol0e8Spec> {
        RegmstwritedonesignalenW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstreaddonesignalen(&mut self) -> RegmstreaddonesignalenW<I3ccontrol0e8Spec> {
        RegmstreaddonesignalenW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstibidonesignalen(&mut self) -> RegmstibidonesignalenW<I3ccontrol0e8Spec> {
        RegmstibidonesignalenW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstddrwritedonesignalen(&mut self) -> RegmstddrwritedonesignalenW<I3ccontrol0e8Spec> {
        RegmstddrwritedonesignalenW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstddrreaddonesignalen(&mut self) -> RegmstddrreaddonesignalenW<I3ccontrol0e8Spec> {
        RegmstddrreaddonesignalenW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regmstinternaldonesignalen(&mut self) -> RegmstinternaldonesignalenW<I3ccontrol0e8Spec> {
        RegmstinternaldonesignalenW::new(self, 10)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_SIGNAL_EN"]
    #[inline(always)]
    pub fn regdetcrstopfailsignalen(&mut self) -> RegdetcrstopfailsignalenW<I3ccontrol0e8Spec> {
        RegdetcrstopfailsignalenW::new(self, 15)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_SIGNAL_EN"]
    #[inline(always)]
    pub fn regrolechangesignalen(&mut self) -> RegrolechangesignalenW<I3ccontrol0e8Spec> {
        RegrolechangesignalenW::new(self, 16)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regsdrbuscdsignalen(&mut self) -> RegsdrbuscdsignalenW<I3ccontrol0e8Spec> {
        RegsdrbuscdsignalenW::new(self, 17)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_SIGNAL_EN"]
    #[inline(always)]
    pub fn regddrbuscdsignalen(&mut self) -> RegddrbuscdsignalenW<I3ccontrol0e8Spec> {
        RegddrbuscdsignalenW::new(self, 18)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_SIGNAL_EN"]
    #[inline(always)]
    pub fn regcrdeqptrdiffsignalen(&mut self) -> RegcrdeqptrdiffsignalenW<I3ccontrol0e8Spec> {
        RegcrdeqptrdiffsignalenW::new(self, 20)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_SIGNAL_EN"]
    #[inline(always)]
    pub fn regibienqptrdiffsignalen(&mut self) -> RegibienqptrdiffsignalenW<I3ccontrol0e8Spec> {
        RegibienqptrdiffsignalenW::new(self, 21)
    }
}
#[doc = "I3C\\_INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0e8Spec;
impl crate::RegisterSpec for I3ccontrol0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0e8::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0e8::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0E8 to value 0"]
impl crate::Resettable for I3ccontrol0e8Spec {}
