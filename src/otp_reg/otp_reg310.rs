#[doc = "Register `OTP_REG310` reader"]
pub type R = crate::R<OtpReg310Spec>;
#[doc = "Register `OTP_REG310` writer"]
pub type W = crate::W<OtpReg310Spec>;
#[doc = "Field `REGSWINFO4` reader - REG_SW_INFO4"]
pub type Regswinfo4R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO4` writer - REG_SW_INFO4"]
pub type Regswinfo4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO4"]
    #[inline(always)]
    pub fn regswinfo4(&self) -> Regswinfo4R {
        Regswinfo4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO4"]
    #[inline(always)]
    pub fn regswinfo4(&mut self) -> Regswinfo4W<OtpReg310Spec> {
        Regswinfo4W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE4\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg310Spec;
impl crate::RegisterSpec for OtpReg310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg310::R`](R) reader structure"]
impl crate::Readable for OtpReg310Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg310::W`](W) writer structure"]
impl crate::Writable for OtpReg310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG310 to value 0x04"]
impl crate::Resettable for OtpReg310Spec {
    const RESET_VALUE: u32 = 0x04;
}
