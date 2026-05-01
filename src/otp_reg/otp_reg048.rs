#[doc = "Register `OTP_REG048` reader"]
pub type R = crate::R<OtpReg048Spec>;
#[doc = "Register `OTP_REG048` writer"]
pub type W = crate::W<OtpReg048Spec>;
#[doc = "Field `REGOTPWDATAM20` reader - REG_OTP_WDATA_M2_0"]
pub type Regotpwdatam20R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM20` writer - REG_OTP_WDATA_M2_0"]
pub type Regotpwdatam20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_0"]
    #[inline(always)]
    pub fn regotpwdatam20(&self) -> Regotpwdatam20R {
        Regotpwdatam20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_0"]
    #[inline(always)]
    pub fn regotpwdatam20(&mut self) -> Regotpwdatam20W<OtpReg048Spec> {
        Regotpwdatam20W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m2\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg048Spec;
impl crate::RegisterSpec for OtpReg048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg048::R`](R) reader structure"]
impl crate::Readable for OtpReg048Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg048::W`](W) writer structure"]
impl crate::Writable for OtpReg048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG048 to value 0"]
impl crate::Resettable for OtpReg048Spec {}
