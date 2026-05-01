#[doc = "Register `OTP_REG094` reader"]
pub type R = crate::R<OtpReg094Spec>;
#[doc = "Register `OTP_REG094` writer"]
pub type W = crate::W<OtpReg094Spec>;
#[doc = "Field `REGOTPWDATAM43` reader - REG_OTP_WDATA_M4_3"]
pub type Regotpwdatam43R = crate::FieldReader<u32>;
#[doc = "Field `REGOTPWDATAM43` writer - REG_OTP_WDATA_M4_3"]
pub type Regotpwdatam43W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_3"]
    #[inline(always)]
    pub fn regotpwdatam43(&self) -> Regotpwdatam43R {
        Regotpwdatam43R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_OTP_WDATA_M4_3"]
    #[inline(always)]
    pub fn regotpwdatam43(&mut self) -> Regotpwdatam43W<OtpReg094Spec> {
        Regotpwdatam43W::new(self, 0)
    }
}
#[doc = "otp\\_wdata\\_m4\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg094Spec;
impl crate::RegisterSpec for OtpReg094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg094::R`](R) reader structure"]
impl crate::Readable for OtpReg094Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg094::W`](W) writer structure"]
impl crate::Writable for OtpReg094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG094 to value 0"]
impl crate::Resettable for OtpReg094Spec {}
