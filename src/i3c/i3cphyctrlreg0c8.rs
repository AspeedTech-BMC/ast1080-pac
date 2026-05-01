#[doc = "Register `I3CPHYCTRLREG0C8` reader"]
pub type R = crate::R<I3cphyctrlreg0c8Spec>;
#[doc = "Register `I3CPHYCTRLREG0C8` writer"]
pub type W = crate::W<I3cphyctrlreg0c8Spec>;
#[doc = "Field `REGPHYMODE` reader - REG_PHY_MODE"]
pub type RegphymodeR = crate::FieldReader;
#[doc = "Field `REGSDRCRTXSTATE` reader - REG_SDR_CR_TX_STATE"]
pub type RegsdrcrtxstateR = crate::FieldReader;
#[doc = "Field `REGSDRCRTXWORDSTATE` reader - REG_SDR_CR_TX_WORD_STATE"]
pub type RegsdrcrtxwordstateR = crate::FieldReader;
#[doc = "Field `REGSDRTGTXSTATE` reader - REG_SDR_TG_TX_STATE"]
pub type RegsdrtgtxstateR = crate::FieldReader;
#[doc = "Field `REGDDRCRTXSTATE` reader - REG_DDR_CR_TX_STATE"]
pub type RegddrcrtxstateR = crate::FieldReader;
#[doc = "Field `REGDDRCRTXWORDSTATE` reader - REG_DDR_CR_TX_WORD_STATE"]
pub type RegddrcrtxwordstateR = crate::FieldReader;
#[doc = "Field `REGRXSTATE` reader - REG_RX_STATE"]
pub type RegrxstateR = crate::FieldReader;
#[doc = "Field `REGRXWORDSTATE` reader - REG_RX_WORD_STATE"]
pub type RegrxwordstateR = crate::FieldReader;
#[doc = "Field `REGBUSCOND` reader - REG_BUS_COND"]
pub type RegbuscondR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - REG_PHY_MODE"]
    #[inline(always)]
    pub fn regphymode(&self) -> RegphymodeR {
        RegphymodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - REG_SDR_CR_TX_STATE"]
    #[inline(always)]
    pub fn regsdrcrtxstate(&self) -> RegsdrcrtxstateR {
        RegsdrcrtxstateR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - REG_SDR_CR_TX_WORD_STATE"]
    #[inline(always)]
    pub fn regsdrcrtxwordstate(&self) -> RegsdrcrtxwordstateR {
        RegsdrcrtxwordstateR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - REG_SDR_TG_TX_STATE"]
    #[inline(always)]
    pub fn regsdrtgtxstate(&self) -> RegsdrtgtxstateR {
        RegsdrtgtxstateR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:18 - REG_DDR_CR_TX_STATE"]
    #[inline(always)]
    pub fn regddrcrtxstate(&self) -> RegddrcrtxstateR {
        RegddrcrtxstateR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21 - REG_DDR_CR_TX_WORD_STATE"]
    #[inline(always)]
    pub fn regddrcrtxwordstate(&self) -> RegddrcrtxwordstateR {
        RegddrcrtxwordstateR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - REG_RX_STATE"]
    #[inline(always)]
    pub fn regrxstate(&self) -> RegrxstateR {
        RegrxstateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:28 - REG_RX_WORD_STATE"]
    #[inline(always)]
    pub fn regrxwordstate(&self) -> RegrxwordstateR {
        RegrxwordstateR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - REG_BUS_COND"]
    #[inline(always)]
    pub fn regbuscond(&self) -> RegbuscondR {
        RegbuscondR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {}
#[doc = "READ\\_PHY\\_STATE\\_MACHINE\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0c8Spec;
impl crate::RegisterSpec for I3cphyctrlreg0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0c8::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0c8::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0C8 to value 0"]
impl crate::Resettable for I3cphyctrlreg0c8Spec {}
