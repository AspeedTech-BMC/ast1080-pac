#[doc = "Register `OTP_REG050` reader"]
pub type R = crate::R<OtpReg050Spec>;
#[doc = "Register `OTP_REG050` writer"]
pub type W = crate::W<OtpReg050Spec>;
#[doc = "Field `REGOTPWDATAM22` reader - REG_OTP_WDATA_M2_2"]
pub type Regotpwdatam22R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM22` writer - REG_OTP_WDATA_M2_2"]
pub type Regotpwdatam22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_2"]
    #[inline(always)]
    pub fn regotpwdatam22(&self) -> Regotpwdatam22R {
        Regotpwdatam22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_2"]
    #[inline(always)]
    pub fn regotpwdatam22(&mut self) -> Regotpwdatam22W<OtpReg050Spec> {
        Regotpwdatam22W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m2\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg050Spec;
impl crate::RegisterSpec for OtpReg050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg050::R`](R) reader structure"]
impl crate::Readable for OtpReg050Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg050::W`](W) writer structure"]
impl crate::Writable for OtpReg050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG050 to value 0"]
impl crate::Resettable for OtpReg050Spec {}
