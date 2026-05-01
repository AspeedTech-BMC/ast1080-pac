#[doc = "Register `OTP_REG1D8` reader"]
pub type R = crate::R<OtpReg1d8Spec>;
#[doc = "Register `OTP_REG1D8` writer"]
pub type W = crate::W<OtpReg1d8Spec>;
#[doc = "Field `REGFWHRIDMSB` reader - REG_FW_HRID_MSB"]
pub type RegfwhridmsbR = crate::FieldReader<u32>;
#[doc = "Field `REGFWHRIDMSB` writer - REG_FW_HRID_MSB"]
pub type RegfwhridmsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_FW_HRID_MSB"]
    #[inline(always)]
    pub fn regfwhridmsb(&self) -> RegfwhridmsbR {
        RegfwhridmsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_FW_HRID_MSB"]
    #[inline(always)]
    pub fn regfwhridmsb(&mut self) -> RegfwhridmsbW<OtpReg1d8Spec> {
        RegfwhridmsbW::new(self, 0)
    }
}
#[doc = "OTP\\_FW\\_ID\\_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1d8Spec;
impl crate::RegisterSpec for OtpReg1d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1d8::R`](R) reader structure"]
impl crate::Readable for OtpReg1d8Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1d8::W`](W) writer structure"]
impl crate::Writable for OtpReg1d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1D8 to value 0"]
impl crate::Resettable for OtpReg1d8Spec {}
