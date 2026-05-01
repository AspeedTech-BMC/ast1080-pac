#[doc = "Register `OTP_REG030` reader"]
pub type R = crate::R<OtpReg030Spec>;
#[doc = "Register `OTP_REG030` writer"]
pub type W = crate::W<OtpReg030Spec>;
#[doc = "Field `REGOTPWDATAM12` reader - REG_OTP_WDATA_M1_2"]
pub type Regotpwdatam12R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM12` writer - REG_OTP_WDATA_M1_2"]
pub type Regotpwdatam12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_2"]
    #[inline(always)]
    pub fn regotpwdatam12(&self) -> Regotpwdatam12R {
        Regotpwdatam12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M1_2"]
    #[inline(always)]
    pub fn regotpwdatam12(&mut self) -> Regotpwdatam12W<OtpReg030Spec> {
        Regotpwdatam12W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m1\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg030Spec;
impl crate::RegisterSpec for OtpReg030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg030::R`](R) reader structure"]
impl crate::Readable for OtpReg030Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg030::W`](W) writer structure"]
impl crate::Writable for OtpReg030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG030 to value 0"]
impl crate::Resettable for OtpReg030Spec {}
