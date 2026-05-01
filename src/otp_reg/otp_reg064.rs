#[doc = "Register `OTP_REG064` reader"]
pub type R = crate::R<OtpReg064Spec>;
#[doc = "Register `OTP_REG064` writer"]
pub type W = crate::W<OtpReg064Spec>;
#[doc = "Field `REGOTPCMDM3` reader - REG_OTP_CMD_M3"]
pub type Regotpcmdm3R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPCMDM3` writer - REG_OTP_CMD_M3"]
pub type Regotpcmdm3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M3"]
    #[inline(always)]
    pub fn regotpcmdm3(&self) -> Regotpcmdm3R {
        Regotpcmdm3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_CMD_M3"]
    #[inline(always)]
    pub fn regotpcmdm3(&mut self) -> Regotpcmdm3W<OtpReg064Spec> {
        Regotpcmdm3W::new(self, 0)
    }
}
#[doc = "otp\\_cmd\\_m3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg064Spec;
impl crate::RegisterSpec for OtpReg064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg064::R`](R) reader structure"]
impl crate::Readable for OtpReg064Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg064::W`](W) writer structure"]
impl crate::Writable for OtpReg064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG064 to value 0"]
impl crate::Resettable for OtpReg064Spec {}
