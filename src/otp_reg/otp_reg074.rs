#[doc = "Register `OTP_REG074` reader"]
pub type R = crate::R<OtpReg074Spec>;
#[doc = "Register `OTP_REG074` writer"]
pub type W = crate::W<OtpReg074Spec>;
#[doc = "Field `REGOTPWDATAM33` reader - REG_OTP_WDATA_M3_3"]
pub type Regotpwdatam33R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM33` writer - REG_OTP_WDATA_M3_3"]
pub type Regotpwdatam33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_3"]
    #[inline(always)]
    pub fn regotpwdatam33(&self) -> Regotpwdatam33R {
        Regotpwdatam33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M3_3"]
    #[inline(always)]
    pub fn regotpwdatam33(&mut self) -> Regotpwdatam33W<OtpReg074Spec> {
        Regotpwdatam33W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m3\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg074Spec;
impl crate::RegisterSpec for OtpReg074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg074::R`](R) reader structure"]
impl crate::Readable for OtpReg074Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg074::W`](W) writer structure"]
impl crate::Writable for OtpReg074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG074 to value 0"]
impl crate::Resettable for OtpReg074Spec {}
