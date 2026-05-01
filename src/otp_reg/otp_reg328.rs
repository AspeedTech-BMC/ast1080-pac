#[doc = "Register `OTP_REG328` reader"]
pub type R = crate::R<OtpReg328Spec>;
#[doc = "Register `OTP_REG328` writer"]
pub type W = crate::W<OtpReg328Spec>;
#[doc = "Field `REGSWINFO10` reader - REG_SW_INFO10"]
pub type Regswinfo10R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO10` writer - REG_SW_INFO10"]
pub type Regswinfo10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO10"]
    #[inline(always)]
    pub fn regswinfo10(&self) -> Regswinfo10R {
        Regswinfo10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO10"]
    #[inline(always)]
    pub fn regswinfo10(&mut self) -> Regswinfo10W<OtpReg328Spec> {
        Regswinfo10W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE10\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg328::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg328::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg328Spec;
impl crate::RegisterSpec for OtpReg328Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg328::R`](R) reader structure"]
impl crate::Readable for OtpReg328Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg328::W`](W) writer structure"]
impl crate::Writable for OtpReg328Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG328 to value 0x10"]
impl crate::Resettable for OtpReg328Spec {
    const RESET_VALUE: u32 = 0x10;
}
