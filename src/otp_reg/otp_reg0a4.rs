#[doc = "Register `OTP_REG0A4` reader"]
pub type R = crate::R<OtpReg0a4Spec>;
#[doc = "Register `OTP_REG0A4` writer"]
pub type W = crate::W<OtpReg0a4Spec>;
#[doc = "Field `REGOTPCMDM5` reader - REG_OTP_CMD_M5"]
pub type Regotpcmdm5R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPCMDM5` writer - REG_OTP_CMD_M5"]
pub type Regotpcmdm5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M5"]
    #[inline(always)]
    pub fn regotpcmdm5(&self) -> Regotpcmdm5R {
        Regotpcmdm5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M5"]
    #[inline(always)]
    pub fn regotpcmdm5(&mut self) -> Regotpcmdm5W<OtpReg0a4Spec> {
        Regotpcmdm5W::new(self, 0)
    }
}
#[doc = "otp\\_cmd\\_m5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0a4Spec;
impl crate::RegisterSpec for OtpReg0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0a4::R`](R) reader structure"]
impl crate::Readable for OtpReg0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0a4::W`](W) writer structure"]
impl crate::Writable for OtpReg0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0A4 to value 0"]
impl crate::Resettable for OtpReg0a4Spec {}
