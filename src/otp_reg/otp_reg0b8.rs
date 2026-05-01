#[doc = "Register `OTP_REG0B8` reader"]
pub type R = crate::R<OtpReg0b8Spec>;
#[doc = "Register `OTP_REG0B8` writer"]
pub type W = crate::W<OtpReg0b8Spec>;
#[doc = "Field `REGOTPSTATUSM5` reader - REG_OTP_STATUS_M5"]
pub type Regotpstatusm5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_STATUS_M5"]
    #[inline(always)]
    pub fn regotpstatusm5(&self) -> Regotpstatusm5R {
        Regotpstatusm5R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_states\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0b8Spec;
impl crate::RegisterSpec for OtpReg0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0b8::R`](R) reader structure"]
impl crate::Readable for OtpReg0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0b8::W`](W) writer structure"]
impl crate::Writable for OtpReg0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0B8 to value 0"]
impl crate::Resettable for OtpReg0b8Spec {}
