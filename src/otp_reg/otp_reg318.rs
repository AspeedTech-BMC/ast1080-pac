#[doc = "Register `OTP_REG318` reader"]
pub type R = crate::R<OtpReg318Spec>;
#[doc = "Register `OTP_REG318` writer"]
pub type W = crate::W<OtpReg318Spec>;
#[doc = "Field `REGSWINFO6` reader - REG_SW_INFO6"]
pub type Regswinfo6R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO6` writer - REG_SW_INFO6"]
pub type Regswinfo6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO6"]
    #[inline(always)]
    pub fn regswinfo6(&self) -> Regswinfo6R {
        Regswinfo6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO6"]
    #[inline(always)]
    pub fn regswinfo6(&mut self) -> Regswinfo6W<OtpReg318Spec> {
        Regswinfo6W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE6\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg318::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg318::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg318Spec;
impl crate::RegisterSpec for OtpReg318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg318::R`](R) reader structure"]
impl crate::Readable for OtpReg318Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg318::W`](W) writer structure"]
impl crate::Writable for OtpReg318Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG318 to value 0x06"]
impl crate::Resettable for OtpReg318Spec {
    const RESET_VALUE: u32 = 0x06;
}
