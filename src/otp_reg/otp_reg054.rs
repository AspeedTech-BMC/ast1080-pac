#[doc = "Register `OTP_REG054` reader"]
pub type R = crate::R<OtpReg054Spec>;
#[doc = "Register `OTP_REG054` writer"]
pub type W = crate::W<OtpReg054Spec>;
#[doc = "Field `REGOTPWDATAM23` reader - REG_OTP_WDATA_M2_3"]
pub type Regotpwdatam23R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM23` writer - REG_OTP_WDATA_M2_3"]
pub type Regotpwdatam23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_3"]
    #[inline(always)]
    pub fn regotpwdatam23(&self) -> Regotpwdatam23R {
        Regotpwdatam23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M2_3"]
    #[inline(always)]
    pub fn regotpwdatam23(&mut self) -> Regotpwdatam23W<OtpReg054Spec> {
        Regotpwdatam23W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m2\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg054Spec;
impl crate::RegisterSpec for OtpReg054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg054::R`](R) reader structure"]
impl crate::Readable for OtpReg054Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg054::W`](W) writer structure"]
impl crate::Writable for OtpReg054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG054 to value 0"]
impl crate::Resettable for OtpReg054Spec {}
