#[doc = "Register `OTP_REG200` reader"]
pub type R = crate::R<OtpReg200Spec>;
#[doc = "Register `OTP_REG200` writer"]
pub type W = crate::W<OtpReg200Spec>;
#[doc = "Field `REGINTREN` reader - REG_INTR_EN"]
pub type RegintrenR = crate::FieldReader<u32>;
#[doc = "Field `REGINTREN` writer - REG_INTR_EN"]
pub type RegintrenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_INTR_EN"]
    #[inline(always)]
    pub fn regintren(&self) -> RegintrenR {
        RegintrenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_INTR_EN"]
    #[inline(always)]
    pub fn regintren(&mut self) -> RegintrenW<OtpReg200Spec> {
        RegintrenW::new(self, 0)
    }
}
#[doc = "otp\\_interrupt\\_en\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg200Spec;
impl crate::RegisterSpec for OtpReg200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg200::R`](R) reader structure"]
impl crate::Readable for OtpReg200Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg200::W`](W) writer structure"]
impl crate::Writable for OtpReg200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG200 to value 0"]
impl crate::Resettable for OtpReg200Spec {}
