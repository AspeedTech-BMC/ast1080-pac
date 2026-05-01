#[doc = "Register `OTP_REG034` reader"]
pub type R = crate::R<OtpReg034Spec>;
#[doc = "Register `OTP_REG034` writer"]
pub type W = crate::W<OtpReg034Spec>;
#[doc = "Field `REGOTPWDATAM13` reader - REG_OTP_WDATA_M1_3"]
pub type Regotpwdatam13R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM13` writer - REG_OTP_WDATA_M1_3"]
pub type Regotpwdatam13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_3"]
    #[inline(always)]
    pub fn regotpwdatam13(&self) -> Regotpwdatam13R {
        Regotpwdatam13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_3"]
    #[inline(always)]
    pub fn regotpwdatam13(&mut self) -> Regotpwdatam13W<OtpReg034Spec> {
        Regotpwdatam13W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m1\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg034Spec;
impl crate::RegisterSpec for OtpReg034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg034::R`](R) reader structure"]
impl crate::Readable for OtpReg034Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg034::W`](W) writer structure"]
impl crate::Writable for OtpReg034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG034 to value 0"]
impl crate::Resettable for OtpReg034Spec {}
