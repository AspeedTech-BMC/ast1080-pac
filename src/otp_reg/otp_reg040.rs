#[doc = "Register `OTP_REG040` reader"]
pub type R = crate::R<OtpReg040Spec>;
#[doc = "Register `OTP_REG040` writer"]
pub type W = crate::W<OtpReg040Spec>;
#[doc = "Field `REGOTPRDATAM1` reader - REG_OTP_RDATA_M1"]
pub type Regotprdatam1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_RDATA_M1"]
    #[inline(always)]
    pub fn regotprdatam1(&self) -> Regotprdatam1R {
        Regotprdatam1R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_rdata\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg040Spec;
impl crate::RegisterSpec for OtpReg040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg040::R`](R) reader structure"]
impl crate::Readable for OtpReg040Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg040::W`](W) writer structure"]
impl crate::Writable for OtpReg040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG040 to value 0"]
impl crate::Resettable for OtpReg040Spec {}
