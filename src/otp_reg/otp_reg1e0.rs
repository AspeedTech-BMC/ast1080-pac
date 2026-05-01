#[doc = "Register `OTP_REG1E0` reader"]
pub type R = crate::R<OtpReg1e0Spec>;
#[doc = "Register `OTP_REG1E0` writer"]
pub type W = crate::W<OtpReg1e0Spec>;
#[doc = "Field `REGCALIPTRARUNTIMESVN0` reader - REG_CALIPTRA_RUNTIME_SVN_0"]
pub type Regcaliptraruntimesvn0R = crate::FieldReader<u32>;
#[doc = "Field `REGCALIPTRARUNTIMESVN0` writer - REG_CALIPTRA_RUNTIME_SVN_0"]
pub type Regcaliptraruntimesvn0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_0"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn0(&self) -> Regcaliptraruntimesvn0R {
        Regcaliptraruntimesvn0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CALIPTRA_RUNTIME_SVN_0"]
    #[inline(always)]
    pub fn regcaliptraruntimesvn0(&mut self) -> Regcaliptraruntimesvn0W<OtpReg1e0Spec> {
        Regcaliptraruntimesvn0W::new(self, 0)
    }
}
#[doc = "OTP\\_CALIP\\_RUNTIME\\_SVN0\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_reg1e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_reg1e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpReg1e0Spec;
impl crate::RegisterSpec for OtpReg1e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_reg1e0::R`](R) reader structure"]
impl crate::Readable for OtpReg1e0Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_reg1e0::W`](W) writer structure"]
impl crate::Writable for OtpReg1e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_REG1E0 to value 0"]
impl crate::Resettable for OtpReg1e0Spec {}
