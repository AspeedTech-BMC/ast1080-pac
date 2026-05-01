#[doc = "Register `OTP_REG0D4` reader"]
pub type R = crate::R<OtpReg0d4Spec>;
#[doc = "Register `OTP_REG0D4` writer"]
pub type W = crate::W<OtpReg0d4Spec>;
#[doc = "Field `REGOTPECCBRPEN` reader - REG_OTP_ECCBRP_EN"]
pub type RegotpeccbrpenR = crate::BitReader;
#[doc = "Field `REGOTPECCBRPEN` writer - REG_OTP_ECCBRP_EN"]
pub type RegotpeccbrpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_OTP_ECCBRP_EN"]
    #[inline(always)]
    pub fn regotpeccbrpen(&self) -> RegotpeccbrpenR {
        RegotpeccbrpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_OTP_ECCBRP_EN"]
    #[inline(always)]
    pub fn regotpeccbrpen(&mut self) -> RegotpeccbrpenW<OtpReg0d4Spec> {
        RegotpeccbrpenW::new(self, 0)
    }
}
#[doc = "otp\\_eccbrp\\_en\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0d4Spec;
impl crate::RegisterSpec for OtpReg0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0d4::R`](R) reader structure"]
impl crate::Readable for OtpReg0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0d4::W`](W) writer structure"]
impl crate::Writable for OtpReg0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0D4 to value 0"]
impl crate::Resettable for OtpReg0d4Spec {}
