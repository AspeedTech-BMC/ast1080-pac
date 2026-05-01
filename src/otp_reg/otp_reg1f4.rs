#[doc = "Register `OTP_REG1F4` reader"]
pub type R = crate::R<OtpReg1f4Spec>;
#[doc = "Register `OTP_REG1F4` writer"]
pub type W = crate::W<OtpReg1f4Spec>;
#[doc = "Field `REGOTPCMDTOCALIPTRAMACRO` reader - REG_OTP_CMD_TO_CALIPTRA_MACRO"]
pub type RegotpcmdtocaliptramacroR = crate::BitReader;
#[doc = "Field `REGOTPCMDTOCALIPTRAMACRO` writer - REG_OTP_CMD_TO_CALIPTRA_MACRO"]
pub type RegotpcmdtocaliptramacroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGOTPCMDTOCALIPTRAMACROWLOCK` reader - REG_OTP_CMD_TO_CALIPTRA_MACRO_WLOCK"]
pub type RegotpcmdtocaliptramacrowlockR = crate::BitReader;
#[doc = "Field `REGOTPCMDTOCALIPTRAMACROWLOCK` writer - REG_OTP_CMD_TO_CALIPTRA_MACRO_WLOCK"]
pub type RegotpcmdtocaliptramacrowlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_OTP_CMD_TO_CALIPTRA_MACRO"]
    #[inline(always)]
    pub fn regotpcmdtocaliptramacro(&self) -> RegotpcmdtocaliptramacroR {
        RegotpcmdtocaliptramacroR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - REG_OTP_CMD_TO_CALIPTRA_MACRO_WLOCK"]
    #[inline(always)]
    pub fn regotpcmdtocaliptramacrowlock(&self) -> RegotpcmdtocaliptramacrowlockR {
        RegotpcmdtocaliptramacrowlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_OTP_CMD_TO_CALIPTRA_MACRO"]
    #[inline(always)]
    pub fn regotpcmdtocaliptramacro(&mut self) -> RegotpcmdtocaliptramacroW<OtpReg1f4Spec> {
        RegotpcmdtocaliptramacroW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_OTP_CMD_TO_CALIPTRA_MACRO_WLOCK"]
    #[inline(always)]
    pub fn regotpcmdtocaliptramacrowlock(
        &mut self,
    ) -> RegotpcmdtocaliptramacrowlockW<OtpReg1f4Spec> {
        RegotpcmdtocaliptramacrowlockW::new(self, 31)
    }
}
#[doc = "OTP\\_CMD\\_SOC2CALIP\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1f4Spec;
impl crate::RegisterSpec for OtpReg1f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1f4::R`](R) reader structure"]
impl crate::Readable for OtpReg1f4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1f4::W`](W) writer structure"]
impl crate::Writable for OtpReg1f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1F4 to value 0"]
impl crate::Resettable for OtpReg1f4Spec {}
