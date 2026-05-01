#[doc = "Register `OTP_REG044` reader"]
pub type R = crate::R<OtpReg044Spec>;
#[doc = "Register `OTP_REG044` writer"]
pub type W = crate::W<OtpReg044Spec>;
#[doc = "Field `REGOTPCMDM2` reader - REG_OTP_CMD_M2"]
pub type Regotpcmdm2R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPCMDM2` writer - REG_OTP_CMD_M2"]
pub type Regotpcmdm2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M2"]
    #[inline(always)]
    pub fn regotpcmdm2(&self) -> Regotpcmdm2R {
        Regotpcmdm2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M2"]
    #[inline(always)]
    pub fn regotpcmdm2(&mut self) -> Regotpcmdm2W<OtpReg044Spec> {
        Regotpcmdm2W::new(self, 0)
    }
}
#[doc = "otp\\_cmd\\_m2\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg044Spec;
impl crate::RegisterSpec for OtpReg044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg044::R`](R) reader structure"]
impl crate::Readable for OtpReg044Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg044::W`](W) writer structure"]
impl crate::Writable for OtpReg044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG044 to value 0"]
impl crate::Resettable for OtpReg044Spec {}
