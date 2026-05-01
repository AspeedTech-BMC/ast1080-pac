#[doc = "Register `OTP_REG188` reader"]
pub type R = crate::R<OtpReg188Spec>;
#[doc = "Register `OTP_REG188` writer"]
pub type W = crate::W<OtpReg188Spec>;
#[doc = "Field `REGRBPCALIPTRASVNREN` reader - REG_RBP_CALIPTRA_SVN_REN"]
pub type RegrbpcaliptrasvnrenR = crate::FieldReader;
#[doc = "Field `REGRBPCALIPTRASVNREN` writer - REG_RBP_CALIPTRA_SVN_REN"]
pub type RegrbpcaliptrasvnrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPCALIPTRASVNWEN` reader - REG_RBP_CALIPTRA_SVN_WEN"]
pub type RegrbpcaliptrasvnwenR = crate::FieldReader;
#[doc = "Field `REGRBPCALIPTRASVNWEN` writer - REG_RBP_CALIPTRA_SVN_WEN"]
pub type RegrbpcaliptrasvnwenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPCALIPTRASVNLOCK` reader - REG_RBP_CALIPTRA_SVN_LOCK"]
pub type RegrbpcaliptrasvnlockR = crate::BitReader;
#[doc = "Field `REGRBPCALIPTRASVNLOCK` writer - REG_RBP_CALIPTRA_SVN_LOCK"]
pub type RegrbpcaliptrasvnlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_RBP_CALIPTRA_SVN_REN"]
    #[inline(always)]
    pub fn regrbpcaliptrasvnren(&self) -> RegrbpcaliptrasvnrenR {
        RegrbpcaliptrasvnrenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_RBP_CALIPTRA_SVN_WEN"]
    #[inline(always)]
    pub fn regrbpcaliptrasvnwen(&self) -> RegrbpcaliptrasvnwenR {
        RegrbpcaliptrasvnwenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_RBP_CALIPTRA_SVN_LOCK"]
    #[inline(always)]
    pub fn regrbpcaliptrasvnlock(&self) -> RegrbpcaliptrasvnlockR {
        RegrbpcaliptrasvnlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_RBP_CALIPTRA_SVN_REN"]
    #[inline(always)]
    pub fn regrbpcaliptrasvnren(&mut self) -> RegrbpcaliptrasvnrenW<OtpReg188Spec> {
        RegrbpcaliptrasvnrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_RBP_CALIPTRA_SVN_WEN"]
    #[inline(always)]
    pub fn regrbpcaliptrasvnwen(&mut self) -> RegrbpcaliptrasvnwenW<OtpReg188Spec> {
        RegrbpcaliptrasvnwenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_RBP_CALIPTRA_SVN_LOCK"]
    #[inline(always)]
    pub fn regrbpcaliptrasvnlock(&mut self) -> RegrbpcaliptrasvnlockW<OtpReg188Spec> {
        RegrbpcaliptrasvnlockW::new(self, 31)
    }
}
#[doc = "OTP\\_RBP\\_CALIP\\_SVN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg188::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg188::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg188Spec;
impl crate::RegisterSpec for OtpReg188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg188::R`](R) reader structure"]
impl crate::Readable for OtpReg188Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg188::W`](W) writer structure"]
impl crate::Writable for OtpReg188Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG188 to value 0xffff"]
impl crate::Resettable for OtpReg188Spec {
    const RESET_VALUE: u32 = 0xffff;
}
