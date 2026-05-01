#[doc = "Register `I3CPHYCTRLREG080` reader"]
pub type R = crate::R<I3cphyctrlreg080Spec>;
#[doc = "Register `I3CPHYCTRLREG080` writer"]
pub type W = crate::W<I3cphyctrlreg080Spec>;
#[doc = "Field `REGSRPPREPARESCLLCNT` reader - REG_SR_P_PREPARE_SCL_LCNT"]
pub type RegsrppreparescllcntR = crate::FieldReader<u16>;
#[doc = "Field `REGSRPPREPARESCLLCNT` writer - REG_SR_P_PREPARE_SCL_LCNT"]
pub type RegsrppreparescllcntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSDAPREPARETRANCNT` reader - REG_SDA_PREPARE_TRAN_CNT"]
pub type RegsdapreparetrancntR = crate::FieldReader<u16>;
#[doc = "Field `REGSDAPREPARETRANCNT` writer - REG_SDA_PREPARE_TRAN_CNT"]
pub type RegsdapreparetrancntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - REG_SR_P_PREPARE_SCL_LCNT"]
    #[inline(always)]
    pub fn regsrppreparescllcnt(&self) -> RegsrppreparescllcntR {
        RegsrppreparescllcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - REG_SDA_PREPARE_TRAN_CNT"]
    #[inline(always)]
    pub fn regsdapreparetrancnt(&self) -> RegsdapreparetrancntR {
        RegsdapreparetrancntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - REG_SR_P_PREPARE_SCL_LCNT"]
    #[inline(always)]
    pub fn regsrppreparescllcnt(&mut self) -> RegsrppreparescllcntW<I3cphyctrlreg080Spec> {
        RegsrppreparescllcntW::new(self, 0)
    }
    #[doc = "Bits 16:26 - REG_SDA_PREPARE_TRAN_CNT"]
    #[inline(always)]
    pub fn regsdapreparetrancnt(&mut self) -> RegsdapreparetrancntW<I3cphyctrlreg080Spec> {
        RegsdapreparetrancntW::new(self, 16)
    }
}
#[doc = "SR\\_P\\_PREPARE\\_SCL\\_SDA\\_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg080Spec;
impl crate::RegisterSpec for I3cphyctrlreg080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg080::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg080Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg080::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG080 to value 0x0004_000b"]
impl crate::Resettable for I3cphyctrlreg080Spec {
    const RESET_VALUE: u32 = 0x0004_000b;
}
