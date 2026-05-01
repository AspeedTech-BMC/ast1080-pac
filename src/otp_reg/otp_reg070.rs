#[doc = "Register `OTP_REG070` reader"]
pub type R = crate::R<OtpReg070Spec>;
#[doc = "Register `OTP_REG070` writer"]
pub type W = crate::W<OtpReg070Spec>;
#[doc = "Field `REGOTPWDATAM32` reader - REG_OTP_WDATA_M3_2"]
pub type Regotpwdatam32R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM32` writer - REG_OTP_WDATA_M3_2"]
pub type Regotpwdatam32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_2"]
    #[inline(always)]
    pub fn regotpwdatam32(&self) -> Regotpwdatam32R {
        Regotpwdatam32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_2"]
    #[inline(always)]
    pub fn regotpwdatam32(&mut self) -> Regotpwdatam32W<OtpReg070Spec> {
        Regotpwdatam32W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m3\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg070Spec;
impl crate::RegisterSpec for OtpReg070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg070::R`](R) reader structure"]
impl crate::Readable for OtpReg070Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg070::W`](W) writer structure"]
impl crate::Writable for OtpReg070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG070 to value 0"]
impl crate::Resettable for OtpReg070Spec {}
