#[doc = "Register `OTP_REG0A0` reader"]
pub type R = crate::R<OtpReg0a0Spec>;
#[doc = "Register `OTP_REG0A0` writer"]
pub type W = crate::W<OtpReg0a0Spec>;
#[doc = "Field `REGOTPRDATAM4` reader - REG_OTP_RDATA_M4"]
pub type Regotprdatam4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_RDATA_M4"]
    #[inline(always)]
    pub fn regotprdatam4(&self) -> Regotprdatam4R {
        Regotprdatam4R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_rdata\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0a0Spec;
impl crate::RegisterSpec for OtpReg0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0a0::R`](R) reader structure"]
impl crate::Readable for OtpReg0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0a0::W`](W) writer structure"]
impl crate::Writable for OtpReg0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0A0 to value 0"]
impl crate::Resettable for OtpReg0a0Spec {}
