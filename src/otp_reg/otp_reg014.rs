#[doc = "Register `OTP_REG014` reader"]
pub type R = crate::R<OtpReg014Spec>;
#[doc = "Register `OTP_REG014` writer"]
pub type W = crate::W<OtpReg014Spec>;
#[doc = "Field `REGOTPWDATAM03` reader - REG_OTP_WDATA_M0_3"]
pub type Regotpwdatam03R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM03` writer - REG_OTP_WDATA_M0_3"]
pub type Regotpwdatam03W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_3"]
    #[inline(always)]
    pub fn regotpwdatam03(&self) -> Regotpwdatam03R {
        Regotpwdatam03R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_3"]
    #[inline(always)]
    pub fn regotpwdatam03(&mut self) -> Regotpwdatam03W<OtpReg014Spec> {
        Regotpwdatam03W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m0\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg014Spec;
impl crate::RegisterSpec for OtpReg014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg014::R`](R) reader structure"]
impl crate::Readable for OtpReg014Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg014::W`](W) writer structure"]
impl crate::Writable for OtpReg014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG014 to value 0"]
impl crate::Resettable for OtpReg014Spec {}
