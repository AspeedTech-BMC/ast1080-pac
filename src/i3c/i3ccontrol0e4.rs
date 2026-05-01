#[doc = "Register `I3CCONTROL0E4` reader"]
pub type R = crate::R<I3ccontrol0e4Spec>;
#[doc = "Register `I3CCONTROL0E4` writer"]
pub type W = crate::W<I3ccontrol0e4Spec>;
#[doc = "Field `REGTGRSTSTATEN` reader - REG_TGRST_STAT_EN"]
pub type RegtgrststatenR = crate::BitReader;
#[doc = "Field `REGTGRSTSTATEN` writer - REG_TGRST_STAT_EN"]
pub type RegtgrststatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVSCLSTUCKSTATEN` reader - REG_SLV_SCL_STUCK_STAT_EN"]
pub type RegslvsclstuckstatenR = crate::BitReader;
#[doc = "Field `REGSLVSCLSTUCKSTATEN` writer - REG_SLV_SCL_STUCK_STAT_EN"]
pub type RegslvsclstuckstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGMSTDAADONESTATEN` reader - REG_MST_DAA_DONE_STAT_EN"]
pub type RegmstdaadonestatenR = crate::BitReader;
#[doc = "Field `REGMSTDAADONESTATEN` writer - REG_MST_DAA_DONE_STAT_EN"]
pub type RegmstdaadonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTWRITEDONESTATEN` reader - REG_MST_WRITE_DONE_STAT_EN"]
pub type RegmstwritedonestatenR = crate::BitReader;
#[doc = "Field `REGMSTWRITEDONESTATEN` writer - REG_MST_WRITE_DONE_STAT_EN"]
pub type RegmstwritedonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTREADDONESTATEN` reader - REG_MST_READ_DONE_STAT_EN"]
pub type RegmstreaddonestatenR = crate::BitReader;
#[doc = "Field `REGMSTREADDONESTATEN` writer - REG_MST_READ_DONE_STAT_EN"]
pub type RegmstreaddonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTIBIDONESTATEN` reader - REG_MST_IBI_DONE_STAT_EN"]
pub type RegmstibidonestatenR = crate::BitReader;
#[doc = "Field `REGMSTIBIDONESTATEN` writer - REG_MST_IBI_DONE_STAT_EN"]
pub type RegmstibidonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRWRITEDONESTATEN` reader - REG_MST_DDR_WRITE_DONE_STAT_EN"]
pub type RegmstddrwritedonestatenR = crate::BitReader;
#[doc = "Field `REGMSTDDRWRITEDONESTATEN` writer - REG_MST_DDR_WRITE_DONE_STAT_EN"]
pub type RegmstddrwritedonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRREADDONESTATEN` reader - REG_MST_DDR_READ_DONE_STAT_EN"]
pub type RegmstddrreaddonestatenR = crate::BitReader;
#[doc = "Field `REGMSTDDRREADDONESTATEN` writer - REG_MST_DDR_READ_DONE_STAT_EN"]
pub type RegmstddrreaddonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTINTERNALDONESTATEN` reader - REG_MST_INTERNAL_DONE_STAT_EN"]
pub type RegmstinternaldonestatenR = crate::BitReader;
#[doc = "Field `REGMSTINTERNALDONESTATEN` writer - REG_MST_INTERNAL_DONE_STAT_EN"]
pub type RegmstinternaldonestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGDETCRSTOPFAILSTATEN` reader - REG_DET_CR_STOP_FAIL_STAT_EN"]
pub type RegdetcrstopfailstatenR = crate::BitReader;
#[doc = "Field `REGDETCRSTOPFAILSTATEN` writer - REG_DET_CR_STOP_FAIL_STAT_EN"]
pub type RegdetcrstopfailstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGROLECHANGESTATEN` reader - REG_ROLE_CHANGE_STAT_EN"]
pub type RegrolechangestatenR = crate::BitReader;
#[doc = "Field `REGROLECHANGESTATEN` writer - REG_ROLE_CHANGE_STAT_EN"]
pub type RegrolechangestatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSDRBUSCDSTATEN` reader - REG_SDR_BUS_CD_STAT_EN"]
pub type RegsdrbuscdstatenR = crate::BitReader;
#[doc = "Field `REGSDRBUSCDSTATEN` writer - REG_SDR_BUS_CD_STAT_EN"]
pub type RegsdrbuscdstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDDRBUSCDSTATEN` reader - REG_DDR_BUS_CD_STAT_EN"]
pub type RegddrbuscdstatenR = crate::BitReader;
#[doc = "Field `REGDDRBUSCDSTATEN` writer - REG_DDR_BUS_CD_STAT_EN"]
pub type RegddrbuscdstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFSTATEN` reader - REG_CR_DEQ_PTR_DIFF_STAT_EN"]
pub type RegcrdeqptrdiffstatenR = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFSTATEN` writer - REG_CR_DEQ_PTR_DIFF_STAT_EN"]
pub type RegcrdeqptrdiffstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIENQPTRDIFFSTATEN` reader - REG_IBI_ENQ_PTR_DIFF_STAT_EN"]
pub type RegibienqptrdiffstatenR = crate::BitReader;
#[doc = "Field `REGIBIENQPTRDIFFSTATEN` writer - REG_IBI_ENQ_PTR_DIFF_STAT_EN"]
pub type RegibienqptrdiffstatenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_TGRST_STAT_EN"]
    #[inline(always)]
    pub fn regtgrststaten(&self) -> RegtgrststatenR {
        RegtgrststatenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_STAT_EN"]
    #[inline(always)]
    pub fn regslvsclstuckstaten(&self) -> RegslvsclstuckstatenR {
        RegslvsclstuckstatenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstdaadonestaten(&self) -> RegmstdaadonestatenR {
        RegmstdaadonestatenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstwritedonestaten(&self) -> RegmstwritedonestatenR {
        RegmstwritedonestatenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstreaddonestaten(&self) -> RegmstreaddonestatenR {
        RegmstreaddonestatenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstibidonestaten(&self) -> RegmstibidonestatenR {
        RegmstibidonestatenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstddrwritedonestaten(&self) -> RegmstddrwritedonestatenR {
        RegmstddrwritedonestatenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstddrreaddonestaten(&self) -> RegmstddrreaddonestatenR {
        RegmstddrreaddonestatenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstinternaldonestaten(&self) -> RegmstinternaldonestatenR {
        RegmstinternaldonestatenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_STAT_EN"]
    #[inline(always)]
    pub fn regdetcrstopfailstaten(&self) -> RegdetcrstopfailstatenR {
        RegdetcrstopfailstatenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_STAT_EN"]
    #[inline(always)]
    pub fn regrolechangestaten(&self) -> RegrolechangestatenR {
        RegrolechangestatenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_STAT_EN"]
    #[inline(always)]
    pub fn regsdrbuscdstaten(&self) -> RegsdrbuscdstatenR {
        RegsdrbuscdstatenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_STAT_EN"]
    #[inline(always)]
    pub fn regddrbuscdstaten(&self) -> RegddrbuscdstatenR {
        RegddrbuscdstatenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_STAT_EN"]
    #[inline(always)]
    pub fn regcrdeqptrdiffstaten(&self) -> RegcrdeqptrdiffstatenR {
        RegcrdeqptrdiffstatenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_STAT_EN"]
    #[inline(always)]
    pub fn regibienqptrdiffstaten(&self) -> RegibienqptrdiffstatenR {
        RegibienqptrdiffstatenR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_TGRST_STAT_EN"]
    #[inline(always)]
    pub fn regtgrststaten(&mut self) -> RegtgrststatenW<I3ccontrol0e4Spec> {
        RegtgrststatenW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_STAT_EN"]
    #[inline(always)]
    pub fn regslvsclstuckstaten(&mut self) -> RegslvsclstuckstatenW<I3ccontrol0e4Spec> {
        RegslvsclstuckstatenW::new(self, 1)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstdaadonestaten(&mut self) -> RegmstdaadonestatenW<I3ccontrol0e4Spec> {
        RegmstdaadonestatenW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstwritedonestaten(&mut self) -> RegmstwritedonestatenW<I3ccontrol0e4Spec> {
        RegmstwritedonestatenW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstreaddonestaten(&mut self) -> RegmstreaddonestatenW<I3ccontrol0e4Spec> {
        RegmstreaddonestatenW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstibidonestaten(&mut self) -> RegmstibidonestatenW<I3ccontrol0e4Spec> {
        RegmstibidonestatenW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstddrwritedonestaten(&mut self) -> RegmstddrwritedonestatenW<I3ccontrol0e4Spec> {
        RegmstddrwritedonestatenW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstddrreaddonestaten(&mut self) -> RegmstddrreaddonestatenW<I3ccontrol0e4Spec> {
        RegmstddrreaddonestatenW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_STAT_EN"]
    #[inline(always)]
    pub fn regmstinternaldonestaten(&mut self) -> RegmstinternaldonestatenW<I3ccontrol0e4Spec> {
        RegmstinternaldonestatenW::new(self, 10)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_STAT_EN"]
    #[inline(always)]
    pub fn regdetcrstopfailstaten(&mut self) -> RegdetcrstopfailstatenW<I3ccontrol0e4Spec> {
        RegdetcrstopfailstatenW::new(self, 15)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_STAT_EN"]
    #[inline(always)]
    pub fn regrolechangestaten(&mut self) -> RegrolechangestatenW<I3ccontrol0e4Spec> {
        RegrolechangestatenW::new(self, 16)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_STAT_EN"]
    #[inline(always)]
    pub fn regsdrbuscdstaten(&mut self) -> RegsdrbuscdstatenW<I3ccontrol0e4Spec> {
        RegsdrbuscdstatenW::new(self, 17)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_STAT_EN"]
    #[inline(always)]
    pub fn regddrbuscdstaten(&mut self) -> RegddrbuscdstatenW<I3ccontrol0e4Spec> {
        RegddrbuscdstatenW::new(self, 18)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_STAT_EN"]
    #[inline(always)]
    pub fn regcrdeqptrdiffstaten(&mut self) -> RegcrdeqptrdiffstatenW<I3ccontrol0e4Spec> {
        RegcrdeqptrdiffstatenW::new(self, 20)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_STAT_EN"]
    #[inline(always)]
    pub fn regibienqptrdiffstaten(&mut self) -> RegibienqptrdiffstatenW<I3ccontrol0e4Spec> {
        RegibienqptrdiffstatenW::new(self, 21)
    }
}
#[doc = "I3C\\_INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0e4Spec;
impl crate::RegisterSpec for I3ccontrol0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0e4::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0e4::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0E4 to value 0"]
impl crate::Resettable for I3ccontrol0e4Spec {}
