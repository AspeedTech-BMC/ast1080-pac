#[doc = "Register `OTP_REG084` reader"]
pub type R = crate::R<OtpReg084Spec>;
#[doc = "Register `OTP_REG084` writer"]
pub type W = crate::W<OtpReg084Spec>;
#[doc = "Field `REGOTPCMDM4` reader - REG_OTP_CMD_M4"]
pub type Regotpcmdm4R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPCMDM4` writer - REG_OTP_CMD_M4"]
pub type Regotpcmdm4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M4"]
    #[inline(always)]
    pub fn regotpcmdm4(&self) -> Regotpcmdm4R {
        Regotpcmdm4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M4"]
    #[inline(always)]
    pub fn regotpcmdm4(&mut self) -> Regotpcmdm4W<OtpReg084Spec> {
        Regotpcmdm4W::new(self, 0)
    }
}
#[doc = "otp\\_cmd\\_m4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg084Spec;
impl crate::RegisterSpec for OtpReg084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg084::R`](R) reader structure"]
impl crate::Readable for OtpReg084Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg084::W`](W) writer structure"]
impl crate::Writable for OtpReg084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG084 to value 0"]
impl crate::Resettable for OtpReg084Spec {}
