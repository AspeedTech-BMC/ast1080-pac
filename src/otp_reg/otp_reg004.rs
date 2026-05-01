#[doc = "Register `OTP_REG004` reader"]
pub type R = crate::R<OtpReg004Spec>;
#[doc = "Register `OTP_REG004` writer"]
pub type W = crate::W<OtpReg004Spec>;
#[doc = "Field `REGOTPCMDM0` reader - REG_OTP_CMD_M0"]
pub type Regotpcmdm0R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPCMDM0` writer - REG_OTP_CMD_M0"]
pub type Regotpcmdm0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M0"]
    #[inline(always)]
    pub fn regotpcmdm0(&self) -> Regotpcmdm0R {
        Regotpcmdm0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M0"]
    #[inline(always)]
    pub fn regotpcmdm0(&mut self) -> Regotpcmdm0W<OtpReg004Spec> {
        Regotpcmdm0W::new(self, 0)
    }
}
#[doc = "otp\\_cmd\\_m0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg004Spec;
impl crate::RegisterSpec for OtpReg004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg004::R`](R) reader structure"]
impl crate::Readable for OtpReg004Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg004::W`](W) writer structure"]
impl crate::Writable for OtpReg004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG004 to value 0"]
impl crate::Resettable for OtpReg004Spec {}
