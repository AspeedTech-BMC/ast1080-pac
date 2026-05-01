#[doc = "Register `OTP_REG090` reader"]
pub type R = crate::R<OtpReg090Spec>;
#[doc = "Register `OTP_REG090` writer"]
pub type W = crate::W<OtpReg090Spec>;
#[doc = "Field `REGOTPWDATAM42` reader - REG_OTP_WDATA_M4_2"]
pub type Regotpwdatam42R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM42` writer - REG_OTP_WDATA_M4_2"]
pub type Regotpwdatam42W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_2"]
    #[inline(always)]
    pub fn regotpwdatam42(&self) -> Regotpwdatam42R {
        Regotpwdatam42R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_2"]
    #[inline(always)]
    pub fn regotpwdatam42(&mut self) -> Regotpwdatam42W<OtpReg090Spec> {
        Regotpwdatam42W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m4\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg090Spec;
impl crate::RegisterSpec for OtpReg090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg090::R`](R) reader structure"]
impl crate::Readable for OtpReg090Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg090::W`](W) writer structure"]
impl crate::Writable for OtpReg090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG090 to value 0"]
impl crate::Resettable for OtpReg090Spec {}
