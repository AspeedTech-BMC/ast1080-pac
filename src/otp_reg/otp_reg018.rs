#[doc = "Register `OTP_REG018` reader"]
pub type R = crate::R<OtpReg018Spec>;
#[doc = "Register `OTP_REG018` writer"]
pub type W = crate::W<OtpReg018Spec>;
#[doc = "Field `REGOTPSTATUSM0` reader - REG_OTP_STATUS_M0"]
pub type Regotpstatusm0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_STATUS_M0"]
    #[inline(always)]
    pub fn regotpstatusm0(&self) -> Regotpstatusm0R {
        Regotpstatusm0R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_states\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg018Spec;
impl crate::RegisterSpec for OtpReg018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg018::R`](R) reader structure"]
impl crate::Readable for OtpReg018Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg018::W`](W) writer structure"]
impl crate::Writable for OtpReg018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG018 to value 0"]
impl crate::Resettable for OtpReg018Spec {}
