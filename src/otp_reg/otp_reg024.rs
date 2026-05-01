#[doc = "Register `OTP_REG024` reader"]
pub type R = crate::R<OtpReg024Spec>;
#[doc = "Register `OTP_REG024` writer"]
pub type W = crate::W<OtpReg024Spec>;
#[doc = "Field `REGOTPCMDM1` reader - REG_OTP_CMD_M1"]
pub type Regotpcmdm1R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPCMDM1` writer - REG_OTP_CMD_M1"]
pub type Regotpcmdm1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M1"]
    #[inline(always)]
    pub fn regotpcmdm1(&self) -> Regotpcmdm1R {
        Regotpcmdm1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M1"]
    #[inline(always)]
    pub fn regotpcmdm1(&mut self) -> Regotpcmdm1W<OtpReg024Spec> {
        Regotpcmdm1W::new(self, 0)
    }
}
#[doc = "otp\\_cmd\\_m1\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg024Spec;
impl crate::RegisterSpec for OtpReg024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg024::R`](R) reader structure"]
impl crate::Readable for OtpReg024Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg024::W`](W) writer structure"]
impl crate::Writable for OtpReg024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG024 to value 0"]
impl crate::Resettable for OtpReg024Spec {}
