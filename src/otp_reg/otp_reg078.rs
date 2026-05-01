#[doc = "Register `OTP_REG078` reader"]
pub type R = crate::R<OtpReg078Spec>;
#[doc = "Register `OTP_REG078` writer"]
pub type W = crate::W<OtpReg078Spec>;
#[doc = "Field `REGOTPSTATUSM3` reader - REG_OTP_STATUS_M3"]
pub type Regotpstatusm3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_STATUS_M3"]
    #[inline(always)]
    pub fn regotpstatusm3(&self) -> Regotpstatusm3R {
        Regotpstatusm3R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_states\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg078Spec;
impl crate::RegisterSpec for OtpReg078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg078::R`](R) reader structure"]
impl crate::Readable for OtpReg078Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg078::W`](W) writer structure"]
impl crate::Writable for OtpReg078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG078 to value 0"]
impl crate::Resettable for OtpReg078Spec {}
