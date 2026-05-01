#[doc = "Register `OTP_REG0B0` reader"]
pub type R = crate::R<OtpReg0b0Spec>;
#[doc = "Register `OTP_REG0B0` writer"]
pub type W = crate::W<OtpReg0b0Spec>;
#[doc = "Field `REGOTPWDATAM52` reader - REG_OTP_WDATA_M5_2"]
pub type Regotpwdatam52R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM52` writer - REG_OTP_WDATA_M5_2"]
pub type Regotpwdatam52W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_2"]
    #[inline(always)]
    pub fn regotpwdatam52(&self) -> Regotpwdatam52R {
        Regotpwdatam52R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M5_2"]
    #[inline(always)]
    pub fn regotpwdatam52(&mut self) -> Regotpwdatam52W<OtpReg0b0Spec> {
        Regotpwdatam52W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m5\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0b0Spec;
impl crate::RegisterSpec for OtpReg0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0b0::R`](R) reader structure"]
impl crate::Readable for OtpReg0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0b0::W`](W) writer structure"]
impl crate::Writable for OtpReg0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0B0 to value 0"]
impl crate::Resettable for OtpReg0b0Spec {}
