#[doc = "Register `OTP_REG0DC` reader"]
pub type R = crate::R<OtpReg0dcSpec>;
#[doc = "Register `OTP_REG0DC` writer"]
pub type W = crate::W<OtpReg0dcSpec>;
#[doc = "Field `REGSWCLRBOOTFSM` reader - REG_SW_CLR_BOOT_FSM"]
pub type RegswclrbootfsmR = crate::BitReader;
#[doc = "Field `REGSWCLRBOOTFSM` writer - REG_SW_CLR_BOOT_FSM"]
pub type RegswclrbootfsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGSWCLRCMDFSM` reader - REG_SW_CLR_CMD_FSM"]
pub type RegswclrcmdfsmR = crate::BitReader;
#[doc = "Field `REGSWCLRCMDFSM` writer - REG_SW_CLR_CMD_FSM"]
pub type RegswclrcmdfsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_SW_CLR_BOOT_FSM"]
    #[inline(always)]
    pub fn regswclrbootfsm(&self) -> RegswclrbootfsmR {
        RegswclrbootfsmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_SW_CLR_CMD_FSM"]
    #[inline(always)]
    pub fn regswclrcmdfsm(&self) -> RegswclrcmdfsmR {
        RegswclrcmdfsmR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_SW_CLR_BOOT_FSM"]
    #[inline(always)]
    pub fn regswclrbootfsm(&mut self) -> RegswclrbootfsmW<OtpReg0dcSpec> {
        RegswclrbootfsmW::new(self, 0)
    }
    #[doc = "Bit 2 - REG_SW_CLR_CMD_FSM"]
    #[inline(always)]
    pub fn regswclrcmdfsm(&mut self) -> RegswclrcmdfsmW<OtpReg0dcSpec> {
        RegswclrcmdfsmW::new(self, 2)
    }
}
#[doc = "otp\\_sw\\_reset\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0dcSpec;
impl crate::RegisterSpec for OtpReg0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0dc::R`](R) reader structure"]
impl crate::Readable for OtpReg0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0dc::W`](W) writer structure"]
impl crate::Writable for OtpReg0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0DC to value 0"]
impl crate::Resettable for OtpReg0dcSpec {}
