#[doc = "Register `OTP_REG058` reader"]
pub type R = crate::R<OtpReg058Spec>;
#[doc = "Register `OTP_REG058` writer"]
pub type W = crate::W<OtpReg058Spec>;
#[doc = "Field `REGOTPSTATUSM2` reader - REG_OTP_STATUS_M2"]
pub type Regotpstatusm2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_STATUS_M2"]
    #[inline(always)]
    pub fn regotpstatusm2(&self) -> Regotpstatusm2R {
        Regotpstatusm2R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_states\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg058Spec;
impl crate::RegisterSpec for OtpReg058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg058::R`](R) reader structure"]
impl crate::Readable for OtpReg058Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg058::W`](W) writer structure"]
impl crate::Writable for OtpReg058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG058 to value 0"]
impl crate::Resettable for OtpReg058Spec {}
