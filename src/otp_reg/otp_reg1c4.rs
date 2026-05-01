#[doc = "Register `OTP_REG1C4` reader"]
pub type R = crate::R<OtpReg1c4Spec>;
#[doc = "Register `OTP_REG1C4` writer"]
pub type W = crate::W<OtpReg1c4Spec>;
#[doc = "Field `REGSECUREBOOTEN` reader - REG_SECURE_BOOT_EN"]
pub type RegsecurebootenR = crate::BitReader;
#[doc = "Field `REGSECUREBOOTEN` writer - REG_SECURE_BOOT_EN"]
pub type RegsecurebootenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSECUREBOOTENWLOCK` reader - REG_SECURE_BOOT_EN_WLOCK"]
pub type RegsecurebootenwlockR = crate::BitReader;
#[doc = "Field `REGSECUREBOOTENWLOCK` writer - REG_SECURE_BOOT_EN_WLOCK"]
pub type RegsecurebootenwlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_SECURE_BOOT_EN"]
    #[inline(always)]
    pub fn regsecurebooten(&self) -> RegsecurebootenR {
        RegsecurebootenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - REG_SECURE_BOOT_EN_WLOCK"]
    #[inline(always)]
    pub fn regsecurebootenwlock(&self) -> RegsecurebootenwlockR {
        RegsecurebootenwlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_SECURE_BOOT_EN"]
    #[inline(always)]
    pub fn regsecurebooten(&mut self) -> RegsecurebootenW<OtpReg1c4Spec> {
        RegsecurebootenW::new(self, 0)
    }
    #[doc = "Bit 31 - REG_SECURE_BOOT_EN_WLOCK"]
    #[inline(always)]
    pub fn regsecurebootenwlock(&mut self) -> RegsecurebootenwlockW<OtpReg1c4Spec> {
        RegsecurebootenwlockW::new(self, 31)
    }
}
#[doc = "OTP\\_SEC\\_BOOT\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1c4Spec;
impl crate::RegisterSpec for OtpReg1c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1c4::R`](R) reader structure"]
impl crate::Readable for OtpReg1c4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1c4::W`](W) writer structure"]
impl crate::Writable for OtpReg1c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1C4 to value 0"]
impl crate::Resettable for OtpReg1c4Spec {}
