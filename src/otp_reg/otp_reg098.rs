#[doc = "Register `OTP_REG098` reader"]
pub type R = crate::R<OtpReg098Spec>;
#[doc = "Register `OTP_REG098` writer"]
pub type W = crate::W<OtpReg098Spec>;
#[doc = "Field `REGOTPSTATUSM4` reader - REG_OTP_STATUS_M4"]
pub type Regotpstatusm4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_STATUS_M4"]
    #[inline(always)]
    pub fn regotpstatusm4(&self) -> Regotpstatusm4R {
        Regotpstatusm4R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_states\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg098Spec;
impl crate::RegisterSpec for OtpReg098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg098::R`](R) reader structure"]
impl crate::Readable for OtpReg098Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg098::W`](W) writer structure"]
impl crate::Writable for OtpReg098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG098 to value 0"]
impl crate::Resettable for OtpReg098Spec {}
