#[doc = "Register `I3CCONTROL0EC` reader"]
pub type R = crate::R<I3ccontrol0ecSpec>;
#[doc = "Register `I3CCONTROL0EC` writer"]
pub type W = crate::W<I3ccontrol0ecSpec>;
#[doc = "Field `REGTGRSTFORCE` reader - REG_TGRST_FORCE"]
pub type RegtgrstforceR = crate::BitReader;
#[doc = "Field `REGTGRSTFORCE` writer - REG_TGRST_FORCE"]
pub type RegtgrstforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVSCLSTUCKFORCE` reader - REG_SLV_SCL_STUCK_FORCE"]
pub type RegslvsclstuckforceR = crate::BitReader;
#[doc = "Field `REGSLVSCLSTUCKFORCE` writer - REG_SLV_SCL_STUCK_FORCE"]
pub type RegslvsclstuckforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGMSTDAADONEFORCE` reader - REG_MST_DAA_DONE_FORCE"]
pub type RegmstdaadoneforceR = crate::BitReader;
#[doc = "Field `REGMSTDAADONEFORCE` writer - REG_MST_DAA_DONE_FORCE"]
pub type RegmstdaadoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTWRITEDONEFORCE` reader - REG_MST_WRITE_DONE_FORCE"]
pub type RegmstwritedoneforceR = crate::BitReader;
#[doc = "Field `REGMSTWRITEDONEFORCE` writer - REG_MST_WRITE_DONE_FORCE"]
pub type RegmstwritedoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTREADDONEFORCE` reader - REG_MST_READ_DONE_FORCE"]
pub type RegmstreaddoneforceR = crate::BitReader;
#[doc = "Field `REGMSTREADDONEFORCE` writer - REG_MST_READ_DONE_FORCE"]
pub type RegmstreaddoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTIBIDONEFORCE` reader - REG_MST_IBI_DONE_FORCE"]
pub type RegmstibidoneforceR = crate::BitReader;
#[doc = "Field `REGMSTIBIDONEFORCE` writer - REG_MST_IBI_DONE_FORCE"]
pub type RegmstibidoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRWRITEDONEFORCE` reader - REG_MST_DDR_WRITE_DONE_FORCE"]
pub type RegmstddrwritedoneforceR = crate::BitReader;
#[doc = "Field `REGMSTDDRWRITEDONEFORCE` writer - REG_MST_DDR_WRITE_DONE_FORCE"]
pub type RegmstddrwritedoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTDDRREADDONEFORCE` reader - REG_MST_DDR_READ_DONE_FORCE"]
pub type RegmstddrreaddoneforceR = crate::BitReader;
#[doc = "Field `REGMSTDDRREADDONEFORCE` writer - REG_MST_DDR_READ_DONE_FORCE"]
pub type RegmstddrreaddoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMSTINTERNALDONEFORCE` reader - REG_MST_INTERNAL_DONE_FORCE"]
pub type RegmstinternaldoneforceR = crate::BitReader;
#[doc = "Field `REGMSTINTERNALDONEFORCE` writer - REG_MST_INTERNAL_DONE_FORCE"]
pub type RegmstinternaldoneforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGDETCRSTOPFAILFORCE` reader - REG_DET_CR_STOP_FAIL_FORCE"]
pub type RegdetcrstopfailforceR = crate::BitReader;
#[doc = "Field `REGDETCRSTOPFAILFORCE` writer - REG_DET_CR_STOP_FAIL_FORCE"]
pub type RegdetcrstopfailforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGROLECHANGEFORCE` reader - REG_ROLE_CHANGE_FORCE"]
pub type RegrolechangeforceR = crate::BitReader;
#[doc = "Field `REGROLECHANGEFORCE` writer - REG_ROLE_CHANGE_FORCE"]
pub type RegrolechangeforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSDRBUSCDFORCE` reader - REG_SDR_BUS_CD_FORCE"]
pub type RegsdrbuscdforceR = crate::BitReader;
#[doc = "Field `REGSDRBUSCDFORCE` writer - REG_SDR_BUS_CD_FORCE"]
pub type RegsdrbuscdforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDDRBUSCDFORCE` reader - REG_DDR_BUS_CD_FORCE"]
pub type RegddrbuscdforceR = crate::BitReader;
#[doc = "Field `REGDDRBUSCDFORCE` writer - REG_DDR_BUS_CD_FORCE"]
pub type RegddrbuscdforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFFORCE` reader - REG_CR_DEQ_PTR_DIFF_FORCE"]
pub type RegcrdeqptrdiffforceR = crate::BitReader;
#[doc = "Field `REGCRDEQPTRDIFFFORCE` writer - REG_CR_DEQ_PTR_DIFF_FORCE"]
pub type RegcrdeqptrdiffforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIENQPTRDIFFFORCE` reader - REG_IBI_ENQ_PTR_DIFF_FORCE"]
pub type RegibienqptrdiffforceR = crate::BitReader;
#[doc = "Field `REGIBIENQPTRDIFFFORCE` writer - REG_IBI_ENQ_PTR_DIFF_FORCE"]
pub type RegibienqptrdiffforceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_TGRST_FORCE"]
    #[inline(always)]
    pub fn regtgrstforce(&self) -> RegtgrstforceR {
        RegtgrstforceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_FORCE"]
    #[inline(always)]
    pub fn regslvsclstuckforce(&self) -> RegslvsclstuckforceR {
        RegslvsclstuckforceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstdaadoneforce(&self) -> RegmstdaadoneforceR {
        RegmstdaadoneforceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstwritedoneforce(&self) -> RegmstwritedoneforceR {
        RegmstwritedoneforceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstreaddoneforce(&self) -> RegmstreaddoneforceR {
        RegmstreaddoneforceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstibidoneforce(&self) -> RegmstibidoneforceR {
        RegmstibidoneforceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstddrwritedoneforce(&self) -> RegmstddrwritedoneforceR {
        RegmstddrwritedoneforceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstddrreaddoneforce(&self) -> RegmstddrreaddoneforceR {
        RegmstddrreaddoneforceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstinternaldoneforce(&self) -> RegmstinternaldoneforceR {
        RegmstinternaldoneforceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_FORCE"]
    #[inline(always)]
    pub fn regdetcrstopfailforce(&self) -> RegdetcrstopfailforceR {
        RegdetcrstopfailforceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_FORCE"]
    #[inline(always)]
    pub fn regrolechangeforce(&self) -> RegrolechangeforceR {
        RegrolechangeforceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_FORCE"]
    #[inline(always)]
    pub fn regsdrbuscdforce(&self) -> RegsdrbuscdforceR {
        RegsdrbuscdforceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_FORCE"]
    #[inline(always)]
    pub fn regddrbuscdforce(&self) -> RegddrbuscdforceR {
        RegddrbuscdforceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_FORCE"]
    #[inline(always)]
    pub fn regcrdeqptrdiffforce(&self) -> RegcrdeqptrdiffforceR {
        RegcrdeqptrdiffforceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_FORCE"]
    #[inline(always)]
    pub fn regibienqptrdiffforce(&self) -> RegibienqptrdiffforceR {
        RegibienqptrdiffforceR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_TGRST_FORCE"]
    #[inline(always)]
    pub fn regtgrstforce(&mut self) -> RegtgrstforceW<I3ccontrol0ecSpec> {
        RegtgrstforceW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_SLV_SCL_STUCK_FORCE"]
    #[inline(always)]
    pub fn regslvsclstuckforce(&mut self) -> RegslvsclstuckforceW<I3ccontrol0ecSpec> {
        RegslvsclstuckforceW::new(self, 1)
    }
    #[doc = "Bit 4 - REG_MST_DAA_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstdaadoneforce(&mut self) -> RegmstdaadoneforceW<I3ccontrol0ecSpec> {
        RegmstdaadoneforceW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_MST_WRITE_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstwritedoneforce(&mut self) -> RegmstwritedoneforceW<I3ccontrol0ecSpec> {
        RegmstwritedoneforceW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_MST_READ_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstreaddoneforce(&mut self) -> RegmstreaddoneforceW<I3ccontrol0ecSpec> {
        RegmstreaddoneforceW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_MST_IBI_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstibidoneforce(&mut self) -> RegmstibidoneforceW<I3ccontrol0ecSpec> {
        RegmstibidoneforceW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_MST_DDR_WRITE_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstddrwritedoneforce(&mut self) -> RegmstddrwritedoneforceW<I3ccontrol0ecSpec> {
        RegmstddrwritedoneforceW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_MST_DDR_READ_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstddrreaddoneforce(&mut self) -> RegmstddrreaddoneforceW<I3ccontrol0ecSpec> {
        RegmstddrreaddoneforceW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_MST_INTERNAL_DONE_FORCE"]
    #[inline(always)]
    pub fn regmstinternaldoneforce(&mut self) -> RegmstinternaldoneforceW<I3ccontrol0ecSpec> {
        RegmstinternaldoneforceW::new(self, 10)
    }
    #[doc = "Bit 15 - REG_DET_CR_STOP_FAIL_FORCE"]
    #[inline(always)]
    pub fn regdetcrstopfailforce(&mut self) -> RegdetcrstopfailforceW<I3ccontrol0ecSpec> {
        RegdetcrstopfailforceW::new(self, 15)
    }
    #[doc = "Bit 16 - REG_ROLE_CHANGE_FORCE"]
    #[inline(always)]
    pub fn regrolechangeforce(&mut self) -> RegrolechangeforceW<I3ccontrol0ecSpec> {
        RegrolechangeforceW::new(self, 16)
    }
    #[doc = "Bit 17 - REG_SDR_BUS_CD_FORCE"]
    #[inline(always)]
    pub fn regsdrbuscdforce(&mut self) -> RegsdrbuscdforceW<I3ccontrol0ecSpec> {
        RegsdrbuscdforceW::new(self, 17)
    }
    #[doc = "Bit 18 - REG_DDR_BUS_CD_FORCE"]
    #[inline(always)]
    pub fn regddrbuscdforce(&mut self) -> RegddrbuscdforceW<I3ccontrol0ecSpec> {
        RegddrbuscdforceW::new(self, 18)
    }
    #[doc = "Bit 20 - REG_CR_DEQ_PTR_DIFF_FORCE"]
    #[inline(always)]
    pub fn regcrdeqptrdiffforce(&mut self) -> RegcrdeqptrdiffforceW<I3ccontrol0ecSpec> {
        RegcrdeqptrdiffforceW::new(self, 20)
    }
    #[doc = "Bit 21 - REG_IBI_ENQ_PTR_DIFF_FORCE"]
    #[inline(always)]
    pub fn regibienqptrdiffforce(&mut self) -> RegibienqptrdiffforceW<I3ccontrol0ecSpec> {
        RegibienqptrdiffforceW::new(self, 21)
    }
}
#[doc = "I3C\\_INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0ecSpec;
impl crate::RegisterSpec for I3ccontrol0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0ec::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0ec::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0EC to value 0"]
impl crate::Resettable for I3ccontrol0ecSpec {}
