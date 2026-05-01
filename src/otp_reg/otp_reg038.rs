#[doc = "Register `OTP_REG038` reader"]
pub type R = crate::R<OtpReg038Spec>;
#[doc = "Register `OTP_REG038` writer"]
pub type W = crate::W<OtpReg038Spec>;
#[doc = "Field `REGOTPSTATUSM1` reader - REG_OTP_STATUS_M1"]
pub type Regotpstatusm1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_STATUS_M1"]
    #[inline(always)]
    pub fn regotpstatusm1(&self) -> Regotpstatusm1R {
        Regotpstatusm1R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_states\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg038Spec;
impl crate::RegisterSpec for OtpReg038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg038::R`](R) reader structure"]
impl crate::Readable for OtpReg038Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg038::W`](W) writer structure"]
impl crate::Writable for OtpReg038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG038 to value 0"]
impl crate::Resettable for OtpReg038Spec {}
