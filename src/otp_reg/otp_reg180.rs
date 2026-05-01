#[doc = "Register `OTP_REG180` reader"]
pub type R = crate::R<OtpReg180Spec>;
#[doc = "Register `OTP_REG180` writer"]
pub type W = crate::W<OtpReg180Spec>;
#[doc = "Field `REGRBPSOCSVNREN` reader - REG_RBP_SOC_SVN_REN"]
pub type RegrbpsocsvnrenR = crate::FieldReader;
#[doc = "Field `REGRBPSOCSVNREN` writer - REG_RBP_SOC_SVN_REN"]
pub type RegrbpsocsvnrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPSOCSVNWEN` reader - REG_RBP_SOC_SVN_WEN"]
pub type RegrbpsocsvnwenR = crate::FieldReader;
#[doc = "Field `REGRBPSOCSVNWEN` writer - REG_RBP_SOC_SVN_WEN"]
pub type RegrbpsocsvnwenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGRBPSOCSVNLOCK` reader - REG_RBP_SOC_SVN_LOCK"]
pub type RegrbpsocsvnlockR = crate::BitReader;
#[doc = "Field `REGRBPSOCSVNLOCK` writer - REG_RBP_SOC_SVN_LOCK"]
pub type RegrbpsocsvnlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - REG_RBP_SOC_SVN_REN"]
    #[inline(always)]
    pub fn regrbpsocsvnren(&self) -> RegrbpsocsvnrenR {
        RegrbpsocsvnrenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_RBP_SOC_SVN_WEN"]
    #[inline(always)]
    pub fn regrbpsocsvnwen(&self) -> RegrbpsocsvnwenR {
        RegrbpsocsvnwenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - REG_RBP_SOC_SVN_LOCK"]
    #[inline(always)]
    pub fn regrbpsocsvnlock(&self) -> RegrbpsocsvnlockR {
        RegrbpsocsvnlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_RBP_SOC_SVN_REN"]
    #[inline(always)]
    pub fn regrbpsocsvnren(&mut self) -> RegrbpsocsvnrenW<OtpReg180Spec> {
        RegrbpsocsvnrenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_RBP_SOC_SVN_WEN"]
    #[inline(always)]
    pub fn regrbpsocsvnwen(&mut self) -> RegrbpsocsvnwenW<OtpReg180Spec> {
        RegrbpsocsvnwenW::new(self, 8)
    }
    #[doc = "Bit 31 - REG_RBP_SOC_SVN_LOCK"]
    #[inline(always)]
    pub fn regrbpsocsvnlock(&mut self) -> RegrbpsocsvnlockW<OtpReg180Spec> {
        RegrbpsocsvnlockW::new(self, 31)
    }
}
#[doc = "OTP\\_RBP\\_SOC\\_SVN\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg180::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg180::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg180Spec;
impl crate::RegisterSpec for OtpReg180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg180::R`](R) reader structure"]
impl crate::Readable for OtpReg180Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg180::W`](W) writer structure"]
impl crate::Writable for OtpReg180Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG180 to value 0xffff"]
impl crate::Resettable for OtpReg180Spec {
    const RESET_VALUE: u32 = 0xffff;
}
