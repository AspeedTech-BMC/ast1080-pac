#[doc = "Register `OTP_REG008` reader"]
pub type R = crate::R<OtpReg008Spec>;
#[doc = "Register `OTP_REG008` writer"]
pub type W = crate::W<OtpReg008Spec>;
#[doc = "Field `REGOTPWDATAM00` reader - REG_OTP_WDATA_M0_0"]
pub type Regotpwdatam00R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM00` writer - REG_OTP_WDATA_M0_0"]
pub type Regotpwdatam00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_0"]
    #[inline(always)]
    pub fn regotpwdatam00(&self) -> Regotpwdatam00R {
        Regotpwdatam00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_0"]
    #[inline(always)]
    pub fn regotpwdatam00(&mut self) -> Regotpwdatam00W<OtpReg008Spec> {
        Regotpwdatam00W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m0\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg008Spec;
impl crate::RegisterSpec for OtpReg008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg008::R`](R) reader structure"]
impl crate::Readable for OtpReg008Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg008::W`](W) writer structure"]
impl crate::Writable for OtpReg008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG008 to value 0"]
impl crate::Resettable for OtpReg008Spec {}
