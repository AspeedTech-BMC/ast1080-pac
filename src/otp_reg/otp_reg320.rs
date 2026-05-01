#[doc = "Register `OTP_REG320` reader"]
pub type R = crate::R<OtpReg320Spec>;
#[doc = "Register `OTP_REG320` writer"]
pub type W = crate::W<OtpReg320Spec>;
#[doc = "Field `REGSWINFO8` reader - REG_SW_INFO8"]
pub type Regswinfo8R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO8` writer - REG_SW_INFO8"]
pub type Regswinfo8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO8"]
    #[inline(always)]
    pub fn regswinfo8(&self) -> Regswinfo8R {
        Regswinfo8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO8"]
    #[inline(always)]
    pub fn regswinfo8(&mut self) -> Regswinfo8W<OtpReg320Spec> {
        Regswinfo8W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE8\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg320::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg320::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg320Spec;
impl crate::RegisterSpec for OtpReg320Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg320::R`](R) reader structure"]
impl crate::Readable for OtpReg320Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg320::W`](W) writer structure"]
impl crate::Writable for OtpReg320Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG320 to value 0x08"]
impl crate::Resettable for OtpReg320Spec {
    const RESET_VALUE: u32 = 0x08;
}
