#[doc = "Register `OTP_REG028` reader"]
pub type R = crate::R<OtpReg028Spec>;
#[doc = "Register `OTP_REG028` writer"]
pub type W = crate::W<OtpReg028Spec>;
#[doc = "Field `REGOTPWDATAM10` reader - REG_OTP_WDATA_M1_0"]
pub type Regotpwdatam10R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM10` writer - REG_OTP_WDATA_M1_0"]
pub type Regotpwdatam10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_0"]
    #[inline(always)]
    pub fn regotpwdatam10(&self) -> Regotpwdatam10R {
        Regotpwdatam10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_0"]
    #[inline(always)]
    pub fn regotpwdatam10(&mut self) -> Regotpwdatam10W<OtpReg028Spec> {
        Regotpwdatam10W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m1\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg028Spec;
impl crate::RegisterSpec for OtpReg028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg028::R`](R) reader structure"]
impl crate::Readable for OtpReg028Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg028::W`](W) writer structure"]
impl crate::Writable for OtpReg028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG028 to value 0"]
impl crate::Resettable for OtpReg028Spec {}
