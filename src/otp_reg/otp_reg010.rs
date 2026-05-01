#[doc = "Register `OTP_REG010` reader"]
pub type R = crate::R<OtpReg010Spec>;
#[doc = "Register `OTP_REG010` writer"]
pub type W = crate::W<OtpReg010Spec>;
#[doc = "Field `REGOTPWDATAM02` reader - REG_OTP_WDATA_M0_2"]
pub type Regotpwdatam02R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM02` writer - REG_OTP_WDATA_M0_2"]
pub type Regotpwdatam02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_2"]
    #[inline(always)]
    pub fn regotpwdatam02(&self) -> Regotpwdatam02R {
        Regotpwdatam02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M0_2"]
    #[inline(always)]
    pub fn regotpwdatam02(&mut self) -> Regotpwdatam02W<OtpReg010Spec> {
        Regotpwdatam02W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m0\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg010Spec;
impl crate::RegisterSpec for OtpReg010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg010::R`](R) reader structure"]
impl crate::Readable for OtpReg010Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg010::W`](W) writer structure"]
impl crate::Writable for OtpReg010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG010 to value 0"]
impl crate::Resettable for OtpReg010Spec {}
