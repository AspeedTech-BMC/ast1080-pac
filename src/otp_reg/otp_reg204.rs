#[doc = "Register `OTP_REG204` reader"]
pub type R = crate::R<OtpReg204Spec>;
#[doc = "Register `OTP_REG204` writer"]
pub type W = crate::W<OtpReg204Spec>;
#[doc = "Field `REGINTRSTS` reader - REG_INTR_STS"]
pub type RegintrstsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_INTR_STS"]
    #[inline(always)]
    pub fn regintrsts(&self) -> RegintrstsR {
        RegintrstsR::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_interrupt\\_status\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg204Spec;
impl crate::RegisterSpec for OtpReg204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg204::R`](R) reader structure"]
impl crate::Readable for OtpReg204Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg204::W`](W) writer structure"]
impl crate::Writable for OtpReg204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG204 to value 0"]
impl crate::Resettable for OtpReg204Spec {}
