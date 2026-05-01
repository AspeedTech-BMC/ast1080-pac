#[doc = "Register `OTP_REG020` reader"]
pub type R = crate::R<OtpReg020Spec>;
#[doc = "Register `OTP_REG020` writer"]
pub type W = crate::W<OtpReg020Spec>;
#[doc = "Field `REGOTPRDATAM0` reader - REG_OTP_RDATA_M0"]
pub type Regotprdatam0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_RDATA_M0"]
    #[inline(always)]
    pub fn regotprdatam0(&self) -> Regotprdatam0R {
        Regotprdatam0R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_rdata\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg020Spec;
impl crate::RegisterSpec for OtpReg020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg020::R`](R) reader structure"]
impl crate::Readable for OtpReg020Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg020::W`](W) writer structure"]
impl crate::Writable for OtpReg020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG020 to value 0"]
impl crate::Resettable for OtpReg020Spec {}
