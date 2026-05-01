#[doc = "Register `I3CCONTROL0E0` reader"]
pub type R = crate::R<I3ccontrol0e0Spec>;
#[doc = "Register `I3CCONTROL0E0` writer"]
pub type W = crate::W<I3ccontrol0e0Spec>;
#[doc = "Field `REGTGRSTSTAT` reader - REG_TGRST_STAT"]
pub type RegtgrststatR = crate::BitReader;
#[doc = "Field `REGTGRSTSTAT` writer - REG_TGRST_STAT"]
pub type RegtgrststatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVSCLSTUCKSTAT` reader - REG_SLV_SCL_STUCK_STAT"]
pub type RegslvsclstuckstatR = crate::BitReader;
#[doc = "Field `REGSLVSCLSTUCKSTAT` writer - REG_SLV_SCL_STUCK_STAT"]
pub type RegslvsclstuckstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGMSTDAADONESTAT` reader - REG_MST_DAA_DONE_STAT"]
pub type RegmstdaadonestatR = crate::BitReader;
#[doc = "Field `REGMSTDAADONESTAT` writer - REG_MST_DAA_DONE_STAT"]
pub type RegmstdaadonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTWRITEDONESTAT` reader - REG_MST_WRITE_DONE_STAT"]
pub type RegmstwritedonestatR = crate::BitReader;
#[doc = "Field `REGMSTWRITEDONESTAT` writer - REG_MST_WRITE_DONE_STAT"]
pub type RegmstwritedonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTREADDONESTAT` reader - REG_MST_READ_DONE_STAT"]
pub type RegmstreaddonestatR = crate::BitReader;
#[doc = "Field `REGMSTREADDONESTAT` writer - REG_MST_READ_DONE_STAT"]
pub type RegmstreaddonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTIBIDONESTAT` reader - REG_MST_IBI_DONE_STAT"]
pub type RegmstibidonestatR = crate::BitReader;
#[doc = "Field `REGMSTIBIDONESTAT` writer - REG_MST_IBI_DONE_STAT"]
pub type RegmstibidonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRWRITEDONESTAT` reader - REG_MST_DDR_WRITE_DONE_STAT"]
pub type RegmstddrwritedonestatR = crate::BitReader;
#[doc = "Field `REGMSTDDRWRITEDONESTAT` writer - REG_MST_DDR_WRITE_DONE_STAT"]
pub type RegmstddrwritedonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRREADDONESTAT` reader - REG_MST_DDR_READ_DONE_STAT"]
pub type RegmstddrreaddonestatR = crate::BitReader;
#[doc = "Field `REGMSTDDRREADDONESTAT` writer - REG_MST_DDR_READ_DONE_STAT"]
pub type RegmstddrreaddonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTINTERNALDONESTAT` reader - REG_MST_INTERNAL_DONE_STAT"]
pub type RegmstinternaldonestatR = crate::BitReader;
#[doc = "Field `REGMSTINTERNALDONESTAT` writer - REG_MST_INTERNAL_DONE_STAT"]
pub type RegmstinternaldonestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGDETCRSTOPFAILSTAT` reader - REG_DET_CR_STOP_FAIL_STAT"]
pub type RegdetcrstopfailstatR = crate::BitReader;
#[doc = "Field `REGDETCRSTOPFAILSTAT` writer - REG_DET_CR_STOP_FAIL_STAT"]
pub type RegdetcrstopfailstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGROLECHANGESTAT` reader - REG_ROLE_CHANGE_STAT"]
pub type RegrolechangestatR = crate::BitReader;
#[doc = "Field `REGROLECHANGESTAT` writer - REG_ROLE_CHANGE_STAT"]
pub type RegrolechangestatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSDRBUSCDSTAT` reader - REG_SDR_BUS_CD_STAT"]
pub type RegsdrbuscdstatR = crate::BitReader;
#[doc = "Field `REGSDRBUSCDSTAT` writer - REG_SDR_BUS_CD_STAT"]
pub type RegsdrbuscdstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDDRBUSCDSTAT` reader - REG_DDR_BUS_CD_STAT"]
pub type RegddrbuscdstatR = crate::BitReader;
#[doc = "Field `REGDDRBUSCDSTAT` writer - REG_DDR_BUS_CD_STAT"]
pub type RegddrbuscdstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFSTAT` reader - REG_CR_DEQ_PTR_DIFF_STAT"]
pub type RegcrdeqptrdiffstatR = crate::BitReader;
#[doc = "Field `REGIBIENQPTRDIFFSTAT` reader - REG_IBI_ENQ_PTR_DIFF_STAT"]
pub type RegibienqptrdiffstatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - REG_TGRST_STAT"]
    #[inline(always)]
    pub fn regtgrststat(&self) -> RegtgrststatR {
        RegtgrststatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_STAT"]
    #[inline(always)]
    pub fn regslvsclstuckstat(&self) -> RegslvsclstuckstatR {
        RegslvsclstuckstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_STAT"]
    #[inline(always)]
    pub fn regmstdaadonestat(&self) -> RegmstdaadonestatR {
        RegmstdaadonestatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_STAT"]
    #[inline(always)]
    pub fn regmstwritedonestat(&self) -> RegmstwritedonestatR {
        RegmstwritedonestatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_STAT"]
    #[inline(always)]
    pub fn regmstreaddonestat(&self) -> RegmstreaddonestatR {
        RegmstreaddonestatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_STAT"]
    #[inline(always)]
    pub fn regmstibidonestat(&self) -> RegmstibidonestatR {
        RegmstibidonestatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_STAT"]
    #[inline(always)]
    pub fn regmstddrwritedonestat(&self) -> RegmstddrwritedonestatR {
        RegmstddrwritedonestatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_STAT"]
    #[inline(always)]
    pub fn regmstddrreaddonestat(&self) -> RegmstddrreaddonestatR {
        RegmstddrreaddonestatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_STAT"]
    #[inline(always)]
    pub fn regmstinternaldonestat(&self) -> RegmstinternaldonestatR {
        RegmstinternaldonestatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_STAT"]
    #[inline(always)]
    pub fn regdetcrstopfailstat(&self) -> RegdetcrstopfailstatR {
        RegdetcrstopfailstatR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_STAT"]
    #[inline(always)]
    pub fn regrolechangestat(&self) -> RegrolechangestatR {
        RegrolechangestatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_STAT"]
    #[inline(always)]
    pub fn regsdrbuscdstat(&self) -> RegsdrbuscdstatR {
        RegsdrbuscdstatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_STAT"]
    #[inline(always)]
    pub fn regddrbuscdstat(&self) -> RegddrbuscdstatR {
        RegddrbuscdstatR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_STAT"]
    #[inline(always)]
    pub fn regcrdeqptrdiffstat(&self) -> RegcrdeqptrdiffstatR {
        RegcrdeqptrdiffstatR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_STAT"]
    #[inline(always)]
    pub fn regibienqptrdiffstat(&self) -> RegibienqptrdiffstatR {
        RegibienqptrdiffstatR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_TGRST_STAT"]
    #[inline(always)]
    pub fn regtgrststat(&mut self) -> RegtgrststatW<I3ccontrol0e0Spec> {
        RegtgrststatW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_STAT"]
    #[inline(always)]
    pub fn regslvsclstuckstat(&mut self) -> RegslvsclstuckstatW<I3ccontrol0e0Spec> {
        RegslvsclstuckstatW::new(self, 1)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_STAT"]
    #[inline(always)]
    pub fn regmstdaadonestat(&mut self) -> RegmstdaadonestatW<I3ccontrol0e0Spec> {
        RegmstdaadonestatW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_STAT"]
    #[inline(always)]
    pub fn regmstwritedonestat(&mut self) -> RegmstwritedonestatW<I3ccontrol0e0Spec> {
        RegmstwritedonestatW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_STAT"]
    #[inline(always)]
    pub fn regmstreaddonestat(&mut self) -> RegmstreaddonestatW<I3ccontrol0e0Spec> {
        RegmstreaddonestatW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_STAT"]
    #[inline(always)]
    pub fn regmstibidonestat(&mut self) -> RegmstibidonestatW<I3ccontrol0e0Spec> {
        RegmstibidonestatW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_STAT"]
    #[inline(always)]
    pub fn regmstddrwritedonestat(&mut self) -> RegmstddrwritedonestatW<I3ccontrol0e0Spec> {
        RegmstddrwritedonestatW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_STAT"]
    #[inline(always)]
    pub fn regmstddrreaddonestat(&mut self) -> RegmstddrreaddonestatW<I3ccontrol0e0Spec> {
        RegmstddrreaddonestatW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_STAT"]
    #[inline(always)]
    pub fn regmstinternaldonestat(&mut self) -> RegmstinternaldonestatW<I3ccontrol0e0Spec> {
        RegmstinternaldonestatW::new(self, 10)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_STAT"]
    #[inline(always)]
    pub fn regdetcrstopfailstat(&mut self) -> RegdetcrstopfailstatW<I3ccontrol0e0Spec> {
        RegdetcrstopfailstatW::new(self, 15)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_STAT"]
    #[inline(always)]
    pub fn regrolechangestat(&mut self) -> RegrolechangestatW<I3ccontrol0e0Spec> {
        RegrolechangestatW::new(self, 16)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_STAT"]
    #[inline(always)]
    pub fn regsdrbuscdstat(&mut self) -> RegsdrbuscdstatW<I3ccontrol0e0Spec> {
        RegsdrbuscdstatW::new(self, 17)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_STAT"]
    #[inline(always)]
    pub fn regddrbuscdstat(&mut self) -> RegddrbuscdstatW<I3ccontrol0e0Spec> {
        RegddrbuscdstatW::new(self, 18)
    }
}
#[doc = "I3C\\_INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0e0Spec;
impl crate::RegisterSpec for I3ccontrol0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0e0::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0e0::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0E0 to value 0"]
impl crate::Resettable for I3ccontrol0e0Spec {}
