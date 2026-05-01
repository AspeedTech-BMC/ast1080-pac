#[doc = "Register `OTP_REG0F4` reader"]
pub type R = crate::R<OtpReg0f4Spec>;
#[doc = "Register `OTP_REG0F4` writer"]
pub type W = crate::W<OtpReg0f4Spec>;
#[doc = "Field `REGPUFRTCTRL` reader - REG_PUFRT_CTRL"]
pub type RegpufrtctrlR = crate::FieldReader<u32>;
#[doc = "Field `REGPUFRTCTRL` writer - REG_PUFRT_CTRL"]
pub type RegpufrtctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_PUFRT_CTRL"]
    #[inline(always)]
    pub fn regpufrtctrl(&self) -> RegpufrtctrlR {
        RegpufrtctrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_PUFRT_CTRL"]
    #[inline(always)]
    pub fn regpufrtctrl(&mut self) -> RegpufrtctrlW<OtpReg0f4Spec> {
        RegpufrtctrlW::new(self, 0)
    }
}
#[doc = "otp\\_puf\\_ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg0f4Spec;
impl crate::RegisterSpec for OtpReg0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg0f4::R`](R) reader structure"]
impl crate::Readable for OtpReg0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg0f4::W`](W) writer structure"]
impl crate::Writable for OtpReg0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG0F4 to value 0"]
impl crate::Resettable for OtpReg0f4Spec {}
