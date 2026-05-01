#[doc = "Register `OTP_REG1D4` reader"]
pub type R = crate::R<OtpReg1d4Spec>;
#[doc = "Register `OTP_REG1D4` writer"]
pub type W = crate::W<OtpReg1d4Spec>;
#[doc = "Field `REGFWHRIDLSB` reader - REG_FW_HRID_LSB"]
pub type RegfwhridlsbR = crate::FieldReader<u32>;
#[doc = "Field `REGFWHRIDLSB` writer - REG_FW_HRID_LSB"]
pub type RegfwhridlsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_FW_HRID_LSB"]
    #[inline(always)]
    pub fn regfwhridlsb(&self) -> RegfwhridlsbR {
        RegfwhridlsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_FW_HRID_LSB"]
    #[inline(always)]
    pub fn regfwhridlsb(&mut self) -> RegfwhridlsbW<OtpReg1d4Spec> {
        RegfwhridlsbW::new(self, 0)
    }
}
#[doc = "OTP\\_FW\\_ID\\_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1d4Spec;
impl crate::RegisterSpec for OtpReg1d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1d4::R`](R) reader structure"]
impl crate::Readable for OtpReg1d4Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1d4::W`](W) writer structure"]
impl crate::Writable for OtpReg1d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1D4 to value 0"]
impl crate::Resettable for OtpReg1d4Spec {}
