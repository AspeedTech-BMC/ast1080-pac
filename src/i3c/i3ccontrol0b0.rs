#[doc = "Register `I3CCONTROL0B0` reader"]
pub type R = crate::R<I3ccontrol0b0Spec>;
#[doc = "Register `I3CCONTROL0B0` writer"]
pub type W = crate::W<I3ccontrol0b0Spec>;
#[doc = "Field `REGI2CDEVICE` reader - REG_I2C_DEVICE"]
pub type Regi2cdeviceR = crate::BitReader;
#[doc = "Field `REGI2CDEVICE` writer - REG_I2C_DEVICE"]
pub type Regi2cdeviceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGNOCCCDEFINING` reader - REG_NO_CCC_DEFINING"]
pub type RegnocccdefiningR = crate::BitReader;
#[doc = "Field `REGNOCCCDEFINING` writer - REG_NO_CCC_DEFINING"]
pub type RegnocccdefiningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `REGIBIWAITAVAILABLE` reader - REG_IBI_WAIT_AVAILABLE"]
pub type RegibiwaitavailableR = crate::BitReader;
#[doc = "Field `REGIBIWAITAVAILABLE` writer - REG_IBI_WAIT_AVAILABLE"]
pub type RegibiwaitavailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMRWAITAVAILABLE` reader - REG_MR_WAIT_AVAILABLE"]
pub type RegmrwaitavailableR = crate::BitReader;
#[doc = "Field `REGMRWAITAVAILABLE` writer - REG_MR_WAIT_AVAILABLE"]
pub type RegmrwaitavailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHJWAITIDLE` reader - REG_HJ_WAIT_IDLE"]
pub type ReghjwaitidleR = crate::BitReader;
#[doc = "Field `REGHJWAITIDLE` writer - REG_HJ_WAIT_IDLE"]
pub type ReghjwaitidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHJWAIT7E` reader - REG_HJ_WAIT_7E"]
pub type Reghjwait7eR = crate::BitReader;
#[doc = "Field `REGHJWAIT7E` writer - REG_HJ_WAIT_7E"]
pub type Reghjwait7eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIREQUEST` reader - REG_IBI_REQUEST"]
pub type RegibirequestR = crate::BitReader;
#[doc = "Field `REGIBIREQUEST` writer - REG_IBI_REQUEST"]
pub type RegibirequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGMRREQUEST` reader - REG_MR_REQUEST"]
pub type RegmrrequestR = crate::BitReader;
#[doc = "Field `REGMRREQUEST` writer - REG_MR_REQUEST"]
pub type RegmrrequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHJREQUEST` reader - REG_HJ_REQUEST"]
pub type ReghjrequestR = crate::BitReader;
#[doc = "Field `REGHJREQUEST` writer - REG_HJ_REQUEST"]
pub type ReghjrequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGACCEPTCR` reader - REG_ACCEPT_CR"]
pub type RegacceptcrR = crate::BitReader;
#[doc = "Field `REGACCEPTCR` writer - REG_ACCEPT_CR"]
pub type RegacceptcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGSLVCCCNORESP` reader - REG_SLV_CCC_NO_RESP"]
pub type RegslvcccnorespR = crate::BitReader;
#[doc = "Field `REGSLVCCCNORESP` writer - REG_SLV_CCC_NO_RESP"]
pub type RegslvcccnorespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSLVCMDATTREN` reader - REG_SLV_CMD_ATTR_EN"]
pub type RegslvcmdattrenR = crate::BitReader;
#[doc = "Field `REGSLVCMDATTREN` writer - REG_SLV_CMD_ATTR_EN"]
pub type RegslvcmdattrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSLVIBIHALTIFERROR` reader - REG_SLV_IBI_HALT_IF_ERROR"]
pub type RegslvibihaltiferrorR = crate::BitReader;
#[doc = "Field `REGSLVIBIHALTIFERROR` writer - REG_SLV_IBI_HALT_IF_ERROR"]
pub type RegslvibihaltiferrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_I2C_DEVICE"]
    #[inline(always)]
    pub fn regi2cdevice(&self) -> Regi2cdeviceR {
        Regi2cdeviceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_NO_CCC_DEFINING"]
    #[inline(always)]
    pub fn regnocccdefining(&self) -> RegnocccdefiningR {
        RegnocccdefiningR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - REG_IBI_WAIT_AVAILABLE"]
    #[inline(always)]
    pub fn regibiwaitavailable(&self) -> RegibiwaitavailableR {
        RegibiwaitavailableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_MR_WAIT_AVAILABLE"]
    #[inline(always)]
    pub fn regmrwaitavailable(&self) -> RegmrwaitavailableR {
        RegmrwaitavailableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_HJ_WAIT_IDLE"]
    #[inline(always)]
    pub fn reghjwaitidle(&self) -> ReghjwaitidleR {
        ReghjwaitidleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_HJ_WAIT_7E"]
    #[inline(always)]
    pub fn reghjwait7e(&self) -> Reghjwait7eR {
        Reghjwait7eR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_IBI_REQUEST"]
    #[inline(always)]
    pub fn regibirequest(&self) -> RegibirequestR {
        RegibirequestR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_MR_REQUEST"]
    #[inline(always)]
    pub fn regmrrequest(&self) -> RegmrrequestR {
        RegmrrequestR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_HJ_REQUEST"]
    #[inline(always)]
    pub fn reghjrequest(&self) -> ReghjrequestR {
        ReghjrequestR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - REG_ACCEPT_CR"]
    #[inline(always)]
    pub fn regacceptcr(&self) -> RegacceptcrR {
        RegacceptcrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - REG_SLV_CCC_NO_RESP"]
    #[inline(always)]
    pub fn regslvcccnoresp(&self) -> RegslvcccnorespR {
        RegslvcccnorespR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - REG_SLV_CMD_ATTR_EN"]
    #[inline(always)]
    pub fn regslvcmdattren(&self) -> RegslvcmdattrenR {
        RegslvcmdattrenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:29 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 30 - REG_SLV_IBI_HALT_IF_ERROR"]
    #[inline(always)]
    pub fn regslvibihaltiferror(&self) -> RegslvibihaltiferrorR {
        RegslvibihaltiferrorR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_I2C_DEVICE"]
    #[inline(always)]
    pub fn regi2cdevice(&mut self) -> Regi2cdeviceW<I3ccontrol0b0Spec> {
        Regi2cdeviceW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_NO_CCC_DEFINING"]
    #[inline(always)]
    pub fn regnocccdefining(&mut self) -> RegnocccdefiningW<I3ccontrol0b0Spec> {
        RegnocccdefiningW::new(self, 1)
    }
    #[doc = "Bit 4 - REG_IBI_WAIT_AVAILABLE"]
    #[inline(always)]
    pub fn regibiwaitavailable(&mut self) -> RegibiwaitavailableW<I3ccontrol0b0Spec> {
        RegibiwaitavailableW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_MR_WAIT_AVAILABLE"]
    #[inline(always)]
    pub fn regmrwaitavailable(&mut self) -> RegmrwaitavailableW<I3ccontrol0b0Spec> {
        RegmrwaitavailableW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_HJ_WAIT_IDLE"]
    #[inline(always)]
    pub fn reghjwaitidle(&mut self) -> ReghjwaitidleW<I3ccontrol0b0Spec> {
        ReghjwaitidleW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_HJ_WAIT_7E"]
    #[inline(always)]
    pub fn reghjwait7e(&mut self) -> Reghjwait7eW<I3ccontrol0b0Spec> {
        Reghjwait7eW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_IBI_REQUEST"]
    #[inline(always)]
    pub fn regibirequest(&mut self) -> RegibirequestW<I3ccontrol0b0Spec> {
        RegibirequestW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_MR_REQUEST"]
    #[inline(always)]
    pub fn regmrrequest(&mut self) -> RegmrrequestW<I3ccontrol0b0Spec> {
        RegmrrequestW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_HJ_REQUEST"]
    #[inline(always)]
    pub fn reghjrequest(&mut self) -> ReghjrequestW<I3ccontrol0b0Spec> {
        ReghjrequestW::new(self, 10)
    }
    #[doc = "Bit 16 - REG_ACCEPT_CR"]
    #[inline(always)]
    pub fn regacceptcr(&mut self) -> RegacceptcrW<I3ccontrol0b0Spec> {
        RegacceptcrW::new(self, 16)
    }
    #[doc = "Bit 20 - REG_SLV_CCC_NO_RESP"]
    #[inline(always)]
    pub fn regslvcccnoresp(&mut self) -> RegslvcccnorespW<I3ccontrol0b0Spec> {
        RegslvcccnorespW::new(self, 20)
    }
    #[doc = "Bit 21 - REG_SLV_CMD_ATTR_EN"]
    #[inline(always)]
    pub fn regslvcmdattren(&mut self) -> RegslvcmdattrenW<I3ccontrol0b0Spec> {
        RegslvcmdattrenW::new(self, 21)
    }
    #[doc = "Bit 30 - REG_SLV_IBI_HALT_IF_ERROR"]
    #[inline(always)]
    pub fn regslvibihaltiferror(&mut self) -> RegslvibihaltiferrorW<I3ccontrol0b0Spec> {
        RegslvibihaltiferrorW::new(self, 30)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0B0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0b0Spec;
impl crate::RegisterSpec for I3ccontrol0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0b0::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0b0::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0B0 to value 0x0001_0000"]
impl crate::Resettable for I3ccontrol0b0Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
