#[doc = "Register `OTP_REG314` reader"]
pub type R = crate::R<OtpReg314Spec>;
#[doc = "Register `OTP_REG314` writer"]
pub type W = crate::W<OtpReg314Spec>;
#[doc = "Field `REGSWINFO5` reader - REG_SW_INFO5"]
pub type Regswinfo5R = crate::FieldReader<u32>;
#[doc = "Field `REGSWINFO5` writer - REG_SW_INFO5"]
pub type Regswinfo5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_SW_INFO5"]
    #[inline(always)]
    pub fn regswinfo5(&self) -> Regswinfo5R {
        Regswinfo5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_SW_INFO5"]
    #[inline(always)]
    pub fn regswinfo5(&mut self) -> Regswinfo5W<OtpReg314Spec> {
        Regswinfo5W::new(self, 0)
    }
}
#[doc = "OTP\\_SW\\_USAGE5\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg314::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg314::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg314Spec;
impl crate::RegisterSpec for OtpReg314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg314::R`](R) reader structure"]
impl crate::Readable for OtpReg314Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg314::W`](W) writer structure"]
impl crate::Writable for OtpReg314Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG314 to value 0x05"]
impl crate::Resettable for OtpReg314Spec {
    const RESET_VALUE: u32 = 0x05;
}
