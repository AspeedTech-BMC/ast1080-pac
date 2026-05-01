#[doc = "Register `OTP_REG300` reader"]
pub type R = crate::R<OtpReg300Spec>;
#[doc = "Register `OTP_REG300` writer"]
pub type W = crate::W<OtpReg300Spec>;
#[doc = "Field `REGSWINFO0` reader - REG_SW_INFO0"]
pub type Regswinfo0R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO0` writer - REG_SW_INFO0"]
pub type Regswinfo0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO0"]
    #[inline(always)]
    pub fn regswinfo0(&self) -> Regswinfo0R {
        Regswinfo0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO0"]
    #[inline(always)]
    pub fn regswinfo0(&mut self) -> Regswinfo0W<OtpReg300Spec> {
        Regswinfo0W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg300::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg300::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg300Spec;
impl crate::RegisterSpec for OtpReg300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg300::R`](R) reader structure"]
impl crate::Readable for OtpReg300Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg300::W`](W) writer structure"]
impl crate::Writable for OtpReg300Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG300 to value 0"]
impl crate::Resettable for OtpReg300Spec {}
