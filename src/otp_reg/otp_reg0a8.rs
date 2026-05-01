#[doc = "Register `OTP_REG0A8` reader"]
pub type R = crate::R<OtpReg0a8Spec>;
#[doc = "Register `OTP_REG0A8` writer"]
pub type W = crate::W<OtpReg0a8Spec>;
#[doc = "Field `REGOTPWDATAM50` reader - REG_OTP_WDATA_M5_0"]
pub type Regotpwdatam50R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM50` writer - REG_OTP_WDATA_M5_0"]
pub type Regotpwdatam50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_0"]
    #[inline(always)]
    pub fn regotpwdatam50(&self) -> Regotpwdatam50R {
        Regotpwdatam50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_0"]
    #[inline(always)]
    pub fn regotpwdatam50(&mut self) -> Regotpwdatam50W<OtpReg0a8Spec> {
        Regotpwdatam50W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m5\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0a8Spec;
impl crate::RegisterSpec for OtpReg0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0a8::R`](R) reader structure"]
impl crate::Readable for OtpReg0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0a8::W`](W) writer structure"]
impl crate::Writable for OtpReg0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0A8 to value 0"]
impl crate::Resettable for OtpReg0a8Spec {}
