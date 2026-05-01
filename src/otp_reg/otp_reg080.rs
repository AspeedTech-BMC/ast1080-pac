#[doc = "Register `OTP_REG080` reader"]
pub type R = crate::R<OtpReg080Spec>;
#[doc = "Register `OTP_REG080` writer"]
pub type W = crate::W<OtpReg080Spec>;
#[doc = "Field `REGOTPRDATAM3` reader - REG_OTP_RDATA_M3"]
pub type Regotprdatam3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_RDATA_M3"]
    #[inline(always)]
    pub fn regotprdatam3(&self) -> Regotprdatam3R {
        Regotprdatam3R::new(self.bits)
    }
}
impl W {}
#[doc = "otp\\_rdata\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg080Spec;
impl crate::RegisterSpec for OtpReg080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg080::R`](R) reader structure"]
impl crate::Readable for OtpReg080Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg080::W`](W) writer structure"]
impl crate::Writable for OtpReg080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG080 to value 0"]
impl crate::Resettable for OtpReg080Spec {}
