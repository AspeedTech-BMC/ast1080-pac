#[doc = "Register `OTP_REG060` reader"]
pub type R = crate::R<OtpReg060Spec>;
#[doc = "Register `OTP_REG060` writer"]
pub type W = crate::W<OtpReg060Spec>;
#[doc = "Field `REGOTPRDATAM2` reader - REG_OTP_RDATA_M2"]
pub type Regotprdatam2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_RDATA_M2"]
    #[inline(always)]
    pub fn regotprdatam2(&self) -> Regotprdatam2R {
        Regotprdatam2R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_rdata\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg060Spec;
impl crate::RegisterSpec for OtpReg060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg060::R`](R) reader structure"]
impl crate::Readable for OtpReg060Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg060::W`](W) writer structure"]
impl crate::Writable for OtpReg060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG060 to value 0"]
impl crate::Resettable for OtpReg060Spec {}
