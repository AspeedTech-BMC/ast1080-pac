#[doc = "Register `OTP_REG088` reader"]
pub type R = crate::R<OtpReg088Spec>;
#[doc = "Register `OTP_REG088` writer"]
pub type W = crate::W<OtpReg088Spec>;
#[doc = "Field `REGOTPWDATAM40` reader - REG_OTP_WDATA_M4_0"]
pub type Regotpwdatam40R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM40` writer - REG_OTP_WDATA_M4_0"]
pub type Regotpwdatam40W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_0"]
    #[inline(always)]
    pub fn regotpwdatam40(&self) -> Regotpwdatam40R {
        Regotpwdatam40R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_0"]
    #[inline(always)]
    pub fn regotpwdatam40(&mut self) -> Regotpwdatam40W<OtpReg088Spec> {
        Regotpwdatam40W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m4\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg088Spec;
impl crate::RegisterSpec for OtpReg088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg088::R`](R) reader structure"]
impl crate::Readable for OtpReg088Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg088::W`](W) writer structure"]
impl crate::Writable for OtpReg088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG088 to value 0"]
impl crate::Resettable for OtpReg088Spec {}
